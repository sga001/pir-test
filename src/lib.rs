#![feature(proc_macro_path_invoc)]
#![feature(plugin, use_extern_macros)]
#![plugin(tarpc_plugins)]

extern crate futures;
extern crate futures_cpupool;
extern crate sealpir;
#[macro_use]
extern crate tarpc;
extern crate xpir;
extern crate time;

pub mod test_sealpir {
    use tarpc::util::Never;
    use sealpir::{PirQuery, PirReply};
    use sealpir::server::PirServer;
    use std::sync::{Arc, RwLock};
    use futures_cpupool::{CpuFuture, CpuPool};
    use futures;
    use time;

    service! {
        rpc set_key(key: Vec<u8>, client_id: u32) -> ();
        rpc get(query: PirQuery, client_id: u32) -> PirReply;
    }

    #[derive(Clone)]
    pub struct PirTestServer {
        handle: Arc<RwLock<PirServer<'static>>>,
        pool: CpuPool,
    }

    impl PirTestServer {
        pub fn new(
            ele_num: u32,
            ele_size: u32,
            poly_degree: u32,
            log_plain_mod: u32,
            d: u32,
        ) -> PirTestServer {
            PirTestServer {
                handle: Arc::new(RwLock::new(PirServer::new(
                    ele_num,
                    ele_size,
                    poly_degree,
                    log_plain_mod,
                    d,
                ))),
                pool: CpuPool::new_num_cpus(),
            }
        }

        pub fn setup<T>(&self, collection: &[T]) {
            let mut handle = self.handle.write().unwrap();
            handle.setup(collection);
        }
    }

    impl FutureService for PirTestServer {
        type GetFut = CpuFuture<PirReply, Never>;

        fn get(&self, query: PirQuery, client_id: u32) -> Self::GetFut {
            let handle_lock = self.handle.clone();

            println!("{:?} req init, client {}", time::get_time(), client_id);

            self.pool.spawn(futures::lazy(move || {
                let handle = handle_lock.read().unwrap(); 
                let reply = handle.gen_reply(&query, client_id);
                println!("{:?} req end, client {}", time::get_time(), client_id);
                Ok(reply)
            }))
        }

        type SetKeyFut = Result<(), Never>;
        fn set_key(&self, key: Vec<u8>, client_id: u32) -> Self::SetKeyFut {
            let mut handle = self.handle.write().unwrap();
            handle.set_galois_key(&key, client_id);
            Ok(())
        }
    }
}

pub mod test_xpir {
    use tarpc::util::Never;
    use xpir::{PirQuery, PirReply};
    use xpir::server::PirServer;
    use std::sync::{Arc, RwLock};
    use futures_cpupool::{CpuFuture, CpuPool};
    use futures;
    use time;

    service! {
        rpc get(query: PirQuery) -> PirReply;
    }

    #[derive(Clone)]
    pub struct PirTestServer {
        handle: Arc<RwLock<PirServer<'static>>>,
        pool: CpuPool,
    }

    impl PirTestServer {
        pub fn new<T>(collection: &[T], alpha: u64, d: u64) -> PirTestServer {
            PirTestServer {
                handle: Arc::new(RwLock::new(PirServer::with_params(collection, alpha, d))),
                pool: CpuPool::new_num_cpus(),
            }
        }
    }

    impl FutureService for PirTestServer {
        type GetFut = CpuFuture<PirReply, Never>;

        fn get(&self, query: PirQuery) -> Self::GetFut {
            let handle_lock = self.handle.clone();

            println!("{:?} req init", time::get_time());

            self.pool.spawn(futures::lazy(move || {
                let handle = handle_lock.read().unwrap();
                let reply = handle.gen_reply(&query);
                println!("{:?} req end", time::get_time());
                Ok(reply)
            }))
        }
    }
}
