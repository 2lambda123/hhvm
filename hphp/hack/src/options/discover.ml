module C = Configurator.V1

let () =
  let filename = "buildOptions.generated.ml" in
  C.main ~name:"build_options" (fun (_ : C.t) ->
      try
        let sysconfdir = Sys.getenv "CMAKE_INSTALL_FULL_SYSCONFDIR"
        and bindir = Sys.getenv "CMAKE_INSTALL_FULL_BINDIR" in
        C.Flags.write_lines
          filename
          [
            "let system_config_path = \"" ^ sysconfdir ^ "\"";
            "let default_hackfmt_path = \"" ^ bindir ^ "/hackfmt\"";
          ]
      with
      | Not_found -> C.Flags.write_lines filename [])
