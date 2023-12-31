(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

module Set_relation = struct
  type t =
    | Equal
    | Subset
    | Superset
    | Disjoint
    | Unknown
end

module type DomainType = sig
  type 'a t

  type ctx

  val relation : 'a t -> ctx:ctx -> 'a t -> Set_relation.t
end

module type S = sig
  module Domain : DomainType

  type 'a t

  val empty : 'a t

  val singleton : 'a Domain.t -> 'a t

  val union : 'a t -> 'a t -> 'a t

  val inter : 'a t -> 'a t -> 'a t

  val diff : 'a t -> 'a t -> 'a t

  val of_list : 'a Domain.t list -> 'a t

  type 'a disjoint =
    | Sat
    | Unsat of {
        left: 'a Domain.t;
        relation: Set_relation.t;
        right: 'a Domain.t;
      }

  val disjoint : Domain.ctx -> 'a t -> 'a t -> 'a disjoint

  val are_disjoint : Domain.ctx -> 'a t -> 'a t -> bool
end

module Make (Domain : DomainType) : S with module Domain := Domain = struct
  type 'a disjoint =
    | Sat
    | Unsat of {
        left: 'a Domain.t;
        relation: Set_relation.t;
        right: 'a Domain.t;
      }

  (* Sets over [Domain.t]; representation is in NNF by construction *)
  module Impl = struct
    type 'a atom = {
      comp: bool;
      elt: 'a Domain.t;
    }

    type 'a t =
      | Set of 'a atom
      | Union of 'a t * 'a t
      | Inter of 'a t * 'a t

    let singleton elt = Set { comp = false; elt }

    let rec disjoint_atom atom1 atom2 ~ctx =
      match (atom1, atom2) with
      | ({ comp = false; elt = elt1 }, { comp = false; elt = elt2 }) ->
        Set_relation.(
          (match Domain.relation ~ctx elt1 elt2 with
          | Disjoint -> Sat
          | (Equal | Subset | Superset | Unknown) as relation ->
            Unsat { left = elt1; relation; right = elt2 }))
      | ({ comp = false; elt = elt1 }, { comp = true; elt = elt2 }) ->
        Set_relation.(
          (* (A disj !B) if A ⊆ B *)
          (match Domain.relation ~ctx elt1 elt2 with
          | Equal
          | Subset ->
            Sat
          | (Superset | Unknown | Disjoint) as relation ->
            Unsat { left = elt1; relation; right = elt2 }))
      | ({ comp = true; elt = elt1 }, { comp = true; elt = elt2 }) ->
        (* Approximation:

           (!A disj !B) iff (A ∪ B) = U && A = !B
             where U := Universal Set (Set Containing All Elements in the Domain)

           There is no way in our model to determine if (A ∪ B) = U holds
           so we are forced to approximate the result. The safest approximation
           is to assume the sets are not disjoint *)
        Unsat { left = elt1; relation = Set_relation.Unknown; right = elt2 }
      | _ -> disjoint_atom atom2 atom1 ~ctx

    let rec disjoint ctx set1 set2 =
      match (set1, set2) with
      (* (L ∪ R) disj S if (L disj S) && (R disj S) *)
      | (Union (l, r), set) -> begin
        match disjoint ctx l set with
        | Sat -> disjoint ctx r set
        | Unsat _ as unsat -> unsat
      end
      (* (L ∩ R) disj S if (L disj S) || (R disj S) *)
      | (Inter (l, r), set) -> begin
        match disjoint ctx l set with
        | Sat -> Sat
        | Unsat _ -> disjoint ctx r set
      end
      | (Set atom1, Set atom2) -> disjoint_atom atom1 atom2 ~ctx
      | (Set _, (Union _ | Inter _)) -> disjoint ctx set2 set1

    let union l r = Union (l, r)

    let inter l r = Inter (l, r)

    (*  Keep values in negation normal form by construction *)
    let rec comp = function
      (* !(!A) = A *)
      | Set atom -> Set { atom with comp = not atom.comp }
      (* De Morgan's Law: !(A ∪ B) = !A ∩ !B *)
      | Union (a, b) -> inter (comp a) (comp b)
      (* De Morgan's Law: !(A ∩ B) = !A ∪ !B *)
      | Inter (a, b) -> union (comp a) (comp b)

    (* A ∖ B = A ∩ !B *)
    let diff a b = inter a (comp b)
  end

  open Impl

  (* We encode an empty set using [Option.None] *)
  type nonrec 'a t = 'a t option

  let empty = None

  let singleton tag = Some (singleton tag)

  let union set1 set2 =
    match (set1, set2) with
    | (set, None)
    | (None, set) ->
      set
    | (Some a, Some b) -> Some (union a b)

  let inter set1 set2 =
    match (set1, set2) with
    | (_, None)
    | (None, _) ->
      None
    | (Some a, Some b) -> Some (inter a b)

  let diff set1 set2 =
    match (set1, set2) with
    | (set, None) -> set
    | (None, _) -> None
    | (Some a, Some b) -> Some (diff a b)

  let of_list elt =
    List.fold_left (fun acc tag -> union acc @@ singleton tag) empty elt

  let disjoint ctx set1 set2 =
    match (set1, set2) with
    | (None, _)
    | (_, None) ->
      Sat
    | (Some set1, Some set2) -> disjoint ctx set1 set2

  let are_disjoint ctx set1 set2 =
    match disjoint ctx set1 set2 with
    | Sat -> true
    | Unsat _ -> false
end
