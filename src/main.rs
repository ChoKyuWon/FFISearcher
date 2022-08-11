#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

use rustc_driver::Compilation;
use rustc_interface::interface::Compiler;
use rustc_interface::Queries;
use rustc_middle::mir::TerminatorKind;
struct MyCallback;

impl MyCallback {
    fn new() -> MyCallback {
        MyCallback {}
    }
}

impl rustc_driver::Callbacks for MyCallback {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            let hir = tcx.hir();
            for id in hir.items() {
                let item = hir.item(id);

                if tcx.is_mir_available(id.def_id) {
                    let mir_body = tcx.optimized_mir(id.def_id);
                    println!("============================================================");

                    for (i, bb) in mir_body.basic_blocks().iter_enumerated() {
                        // Statement traversal
                        // for statement in &bb.statements {
                        //     print!("{:?}", statement);
                        //     if let rustc_middle::mir::ClearCrossCrate::Set(source_scope_local_data) = &mir_body.source_scopes[statement.source_info.scope].local_data {
                        //         if source_scope_local_data.safety == rustc_middle::mir::Safety::Safe {
                        //             print!("\t\tsafe");
                        //         } else {
                        //             print!("\t\tunsafe");
                        //         }
                        //     }
                        //     println!();
                        //     println!("{:?}", statement.source_info.scope);
                        // }
                        let terminator = &bb.terminator.as_ref().unwrap();
                        // if let rustc_middle::mir::ClearCrossCrate::Set(source_scope_local_data) = &mir_body.source_scopes[terminator.source_info.scope].local_data {
                        //     if source_scope_local_data.safety == rustc_middle::mir::Safety::Safe {
                        //         print!("\t\tsafe");
                        //     } else {
                        //         print!("\t\tunsafe");
                        //     }
                        // }
                        let f_did = match &terminator.kind {
                            TerminatorKind::Call { func, args, .. } => {
                                let opt = func.const_fn_def().unwrap().0.as_local();
                                if opt == None {
                                    None
                                } else {
                                    Some((func, opt.unwrap(), args))
                                }
                            }
                            _ => None,
                        };
                        match f_did {
                            Some((fname, did, arg)) => {
                                println!("----- bb {} -----", i.index());
                                println!("{:?}, {}, {:?}", fname, tcx.is_foreign_item(did), arg);
                            }
                            None => {}
                        }
                    }
                }
            }
        });

        Compilation::Continue
    }
}

fn main() {
    //    let mut callbacks = rustc_driver::TimePassesCallbacks::default();
    let mut callbacks = MyCallback::new();

    let rustc_args = vec![
        "run".to_string(),
        "FFI.rs".to_string(),
        "--sysroot".to_string(),
        "/home/kyuwoncho18/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu".to_string(),
    ];
    let run_compiler = rustc_driver::RunCompiler::new(&rustc_args, &mut callbacks);
    run_compiler.run();
}
