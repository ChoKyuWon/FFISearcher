#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_target;

use rustc_driver::Compilation;
use rustc_hir::{ForeignItemId, ItemKind};
use rustc_interface::interface::Compiler;
use rustc_interface::Queries;
use rustc_middle::mir::TerminatorKind;
use rustc_target::spec::abi::Abi;
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
            let mut FFI_id: Vec<ForeignItemId> = Vec::new();

            // In this step, we try to find all FFI items
            for id in hir.items() {
                let item = hir.item(id);
                if let ItemKind::ForeignMod { abi, items } = item.kind {
                    if let Abi::C { .. } = abi {
                        // Now we found FFI function decl;
                        for i in items {
                            FFI_id.push(i.id);
                        }
                    }
                }
            }
            println!("{:?}", FFI_id);

            // In this step, we try to find all FFI usage
            for id in hir.items() {
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
                        if let Some((fname, did, args)) = f_did {
                            println!("----- bb {} -----", i.index());
                            println!("{:?}", terminator.source_info.span);
                            println!("{:?}, {}, {:?}", fname, tcx.is_foreign_item(did), args);
                            for (i, arg) in args.into_iter().enumerate() {
                                let p = arg.place();
                                if p == None {
                                    continue;
                                }
                                let t = arg.ty(mir_body, tcx);
                                if t.is_unsafe_ptr() {
                                    println!("Unsafe ptr: {:?}, {}th arg", t, i + 1);
                                }
                            }
                            // println!("{:?}", hir.fn_decl_by_hir_id(hir.local_def_id_to_hir_id(did)));
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
