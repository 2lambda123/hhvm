/* Copyright (c) 2006, 2019, Oracle and/or its affiliates. All rights reserved.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License, version 2.0,
   as published by the Free Software Foundation.

   This program is also distributed with certain software (including
   but not limited to OpenSSL) that is licensed under separate terms,
   as designated in a particular file or component or in included license
   documentation.  The authors of MySQL hereby grant you an additional
   permission to link the program and your derivative works with the
   separately licensed software that they have included with MySQL.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License, version 2.0, for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301  USA */

#ifndef SQL_INSERT_INCLUDED
#define SQL_INSERT_INCLUDED

#include <stddef.h>
#include <sys/types.h>

#include "my_dbug.h"
#include "my_inttypes.h"
#include "my_sqlcommand.h"
#include "sql/query_result.h"     // Query_result_interceptor
#include "sql/sql_cmd_dml.h"      // Sql_cmd_dml
#include "sql/sql_data_change.h"  // enum_duplicates
#include "sql/sql_list.h"
#include "sql/table.h"

class Alter_info;
class Field;
class Item;
class SELECT_LEX_UNIT;
class THD;
struct HA_CREATE_INFO;
struct handlerton;

typedef List<Item> List_item;
struct MYSQL_LOCK;

bool check_that_all_fields_are_given_values(THD *thd, TABLE *entry,
                                            TABLE_LIST *table_list);
bool write_record(THD *thd, TABLE *table, COPY_INFO *info, COPY_INFO *update);
bool validate_default_values_of_unset_fields(THD *thd, TABLE *table);

class Query_result_insert : public Query_result_interceptor {
 public:
  /// The table used for insertion of rows
  TABLE_LIST *table_list;
  TABLE *table;

 private:
  /**
     The columns of the table to be inserted into, *or* the columns of the
     table from which values are selected. For legacy reasons both are
     allowed.
   */
  List<Item> *fields;

 protected:
  /// ha_start_bulk_insert has been called.
  bool bulk_insert_started;

 public:
  ulonglong autoinc_value_of_last_inserted_row;  // autogenerated or not
  COPY_INFO info;
  COPY_INFO update;  ///< the UPDATE part of "info"
  bool insert_into_view;

  /**
     Creates a Query_result_insert for routing a result set to an existing
     table.

     @param table_list_par   The table reference for the destination table.
     @param table_par        The destination table. May be NULL.
     @param target_columns   See details.
     @param target_or_source_columns See details.
     @param update_fields    The columns to be updated in case of duplicate
                             keys. May be NULL.
     @param update_values    The values to be assigned in case of duplicate
                             keys. May be NULL.
     @param duplic           The policy for handling duplicates.

     @todo This constructor takes 8 arguments, 6 of which are used to
     immediately construct a COPY_INFO object. Obviously the constructor
     should take the COPY_INFO object as argument instead. Also, some
     Query_result_insert members initialized here are totally redundant, as they
are found inside the COPY_INFO.

     The target_columns and target_or_source_columns arguments are set by
     callers as follows:
     @li if CREATE SELECT:
      - target_columns == NULL,
      - target_or_source_columns == expressions listed after SELECT, as in
          CREATE ... SELECT expressions
     @li if INSERT SELECT:
      target_columns
      == target_or_source_columns
      == columns listed between INSERT and SELECT, as in
          INSERT INTO t (columns) SELECT ...

     We set the manage_defaults argument of info's constructor as follows
     ([...] denotes something optional):
     @li If target_columns==NULL, the statement is
@verbatim
     CREATE TABLE a_table [(columns1)] SELECT expressions2
@endverbatim
     so 'info' must manage defaults of columns1.
     @li Otherwise it is:
@verbatim
     INSERT INTO a_table [(columns1)] SELECT ...
@endverbatim
     target_columns is columns1, if not empty then 'info' must manage defaults
     of other columns than columns1.
  */
  Query_result_insert(TABLE_LIST *table_list_par, TABLE *table_par,
                      List<Item> *target_columns,
                      List<Item> *target_or_source_columns,
                      List<Item> *update_fields, List<Item> *update_values,
                      enum_duplicates duplic)
      : Query_result_interceptor(),
        table_list(table_list_par),
        table(table_par),
        fields(target_or_source_columns),
        bulk_insert_started(false),
        autoinc_value_of_last_inserted_row(0),
        info(COPY_INFO::INSERT_OPERATION, target_columns,
             // manage_defaults
             (target_columns == nullptr || target_columns->elements != 0),
             duplic),
        update(COPY_INFO::UPDATE_OPERATION, update_fields, update_values),
        insert_into_view(table_list_par && table_list_par->is_view()) {
    DBUG_ASSERT(target_or_source_columns != nullptr);
    DBUG_ASSERT(target_columns == target_or_source_columns ||
                target_columns == nullptr);
  }

 public:
  bool need_explain_interceptor() const override { return true; }
  bool prepare(THD *thd, List<Item> &list, SELECT_LEX_UNIT *u) override;
  bool start_execution(THD *thd) override;
  bool send_data(THD *thd, List<Item> &items) override;
  virtual void store_values(THD *thd, List<Item> &values);
  void send_error(THD *thd, uint errcode, const char *err) override;
  bool send_eof(THD *thd) override;
  void abort_result_set(THD *thd) override;
  void cleanup(THD *thd) override;

 private:
  /**
    Indicates whether this statement should be written to binary log's
    transactional cache in statement mode.
  */
  virtual bool stmt_binlog_is_trans() const;
};

/**
   @todo This class inherits a class which is non-abstract. This is not in
   line with good programming practices and the inheritance should be broken
   up.
*/
class Query_result_create final : public Query_result_insert {
  TABLE_LIST *create_table;
  HA_CREATE_INFO *create_info;
  TABLE_LIST *select_tables;
  Alter_info *alter_info;
  Field **field;
  /* lock data for tmp table */
  MYSQL_LOCK *m_lock;
  /* m_lock or thd->extra_lock */
  MYSQL_LOCK **m_plock;
  /**
    If table being created has SE supporting atomic DDL, pointer to SE's
    handlerton object to be used for calling SE post-DDL hook, nullptr -
    otherwise.
  */
  handlerton *m_post_ddl_ht;

 public:
  Query_result_create(TABLE_LIST *table_arg, HA_CREATE_INFO *create_info_par,
                      Alter_info *alter_info_arg, List<Item> &select_fields,
                      enum_duplicates duplic, TABLE_LIST *select_tables_arg);

  bool prepare(THD *thd, List<Item> &list, SELECT_LEX_UNIT *u) override;
  void store_values(THD *thd, List<Item> &values) override;
  void send_error(THD *thd, uint errcode, const char *err) override;
  bool send_eof(THD *thd) override;
  void abort_result_set(THD *thd) override;
  bool start_execution(THD *thd) override;

 private:
  bool stmt_binlog_is_trans() const override;
  int binlog_show_create_table(THD *thd);
  void drop_open_table(THD *thd);
};

/**
  Base class for all INSERT and REPLACE statements. Abstract class that
  is inherited by Sql_cmd_insert_values and Sql_cmd_insert_select.
*/

class Sql_cmd_insert_base : public Sql_cmd_dml {
 protected:
  virtual bool precheck(THD *thd) override;

  virtual bool prepare_inner(THD *thd) override;

 private:
  bool resolve_update_expressions(THD *thd);
  bool prepare_values_table(THD *thd);
  bool resolve_values_table_columns(THD *thd);

 public:
  /*
    field_list was created for view and should be removed before PS/SP
    rexecuton
  */
  bool empty_field_list_on_rset;

 protected:
  const bool is_replace;

 public:
  /**
    Field list to insert/replace

    One of two things:
    1. For the INSERT/REPLACE ... (col1, ... colN) VALUES ... syntax
       this is a list of col1, ..., colN fields.
    2. For the INSERT/REPLACE ... SET col1=x1, ... colM=xM syntax extension
       this is a list of col1, ... colM fields as well.
  */
  List<Item> insert_field_list;
  /**
    Row data to insert/replace

    One of two things:
    1. For the INSERT/REPLACE ... VALUES (row1), (row2), ... (rowN) syntax
       the list contains N List_item lists: one List_item per row.
    2. For the INSERT/REPLACE ... SET col1=x1, ... colM=xM syntax extension
       this list contains only 1 List_item of M data values: this way we
       emulate this syntax:
         INSERT/REPLACE ... (col1, ... colM) VALUE (x1, ..., xM);
  */
  List<List_item> insert_many_values;
  /// Number of values per row in insert_many_values, available after resolving
  uint value_count;

  /// ON DUPLICATE KEY UPDATE field list
  List<Item> update_field_list;

  /// ON DUPLICATE KEY UPDATE data value list
  List<Item> update_value_list;

  /**
    ON DUPLICATE KEY UPDATE reference to VALUES.. as a derived table.
  */
  TABLE_LIST *values_table{nullptr};
  Create_col_name_list *values_column_list{nullptr};

  /**
    Field list for VALUES derived table. If no insert_field exists (e.g. INSERT
    INTO t0 ..), we have to create one to create Item_insert_values for ODKU
    statements.
  */
  List<Item> values_field_list;

  const enum_duplicates duplicates;

  explicit Sql_cmd_insert_base(bool is_replace_arg,
                               enum_duplicates duplicates_arg)
      : empty_field_list_on_rset(false),
        is_replace(is_replace_arg),
        value_count(0),
        duplicates(duplicates_arg) {}

  virtual void cleanup(THD *) override {
    if (empty_field_list_on_rset) {
      empty_field_list_on_rset = false;
      insert_field_list.empty();
    }
  }

  virtual bool accept(THD *thd, Select_lex_visitor *visitor) override;
};

/**
  Class that implements INSERT ... VALUES and REPLACE ... VALUES statements.
*/

class Sql_cmd_insert_values : public Sql_cmd_insert_base {
 public:
  explicit Sql_cmd_insert_values(bool is_replace_arg,
                                 enum_duplicates duplicates_arg)
      : Sql_cmd_insert_base(is_replace_arg, duplicates_arg) {}

  virtual enum_sql_command sql_command_code() const {
    return is_replace ? SQLCOM_REPLACE : SQLCOM_INSERT;
  }

  virtual bool is_single_table_plan() const { return true; }

 protected:
  virtual bool execute_inner(THD *thd);
};

/**
  Class that implements INSERT ... SELECT and REPLACE ... SELECT statements.
*/

class Sql_cmd_insert_select : public Sql_cmd_insert_base {
 public:
  explicit Sql_cmd_insert_select(bool is_replace_arg,
                                 enum_duplicates duplicates_arg)
      : Sql_cmd_insert_base(is_replace_arg, duplicates_arg) {}

  virtual enum_sql_command sql_command_code() const {
    return is_replace ? SQLCOM_REPLACE_SELECT : SQLCOM_INSERT_SELECT;
  }
};

#endif /* SQL_INSERT_INCLUDED */
