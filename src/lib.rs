extern crate curl;
extern crate json;
extern crate rustc_serialize;
extern crate time;

mod coder;

use std::path::Path;
use curl::easy::{Easy, Form};

fn parse_bool(inp: bool) -> String {
    match inp {
        true => "true".to_string(),
        _ => "false".to_string(),
    }
}

#[derive(Default)]
pub struct IPFS {
    host: String,
    port: u16,
    url: String,
    args: String,
    path: String,
}

impl IPFS {

    pub fn new() -> IPFS {
        let ipfs: IPFS = Default::default();
        ipfs
    }

    pub fn host(&mut self, inp: &str, port: u16) -> &mut IPFS {
        self.host = inp.to_string();
        self.port = port;
        self
    }

    fn url(&mut self, inp: &str) -> &mut IPFS {
        self.url = inp.to_string();
        self
    }

    fn args(&mut self, inp: &str) -> &mut IPFS {
        self.args = inp.to_string();
        self
    }

    fn path(&mut self, p: &str) -> &mut IPFS {
        self.path = p.to_string();
        self
    }


    //--------------------------------------- API FUNCTIONS ---------------------------------------


    pub fn add(&mut self, raw_path: &str) -> Vec<u8> {
        self.url("/api/v0/add").path(raw_path).query_post_upload()
    }

    pub fn bitswap_ledger(&mut self, peer: &str) -> Vec<u8> {
        let l = format!("arg={}", peer);
        self.url("/api/v0/bitswap/ledger").args(&l).query()
    }

    pub fn bitswap_stat(&mut self) -> Vec<u8> {
        self.url("/api/v0/bitswap/stat").query()
    }

    pub fn bitswap_unwant(&mut self, key: &str) -> Vec<u8> {
        let l = format!("arg={}", key);
        self.url("/api/v0/bitswap/unwant").args(&l).query()
    }

    pub fn bitswap_wantlist(&mut self, peer: &str) -> Vec<u8> {
        let l = format!("arg={}", peer);
        self.url("/api/v0/bitswap/wantlist").args(&l).query()
    }

    pub fn block_get(&mut self, hash: &str) -> Vec<u8> {
        let l = format!("arg={}", hash);
        self.url("/api/v0/block/get").args(&l).query()
    }

    pub fn block_put(&mut self, raw_path: &str, fmt: &str, mhtype: &str, mhlen: i64) -> Vec<u8> {
        let l = format!("format={}&mhtype={}&mhlen={}", fmt, mhtype, mhlen);
        self.url("/api/v0/block/put").args(&l).path(raw_path).query_post_upload()
    }

    pub fn block_rm(&mut self, hash: &str, force: bool, quiet: bool) -> Vec<u8> {
        let f = &parse_bool(force);
        let q = &parse_bool(quiet);
        let l = format!("arg={}&force={}&quiet={}", hash, f, q);
        self.url("/api/v0/block/rm").args(&l).query()
    }

    pub fn block_stat(&mut self, hash: &str) -> Vec<u8> {
        let l = format!("arg={}", hash);
        self.url("/api/v0/block/stat").args(&l).query()
    }

    pub fn bootstrap_add_default(&mut self) -> Vec<u8> {
        self.url("/api/v0/bootstrap/add/default").query()
    }

    pub fn bootstrap_list(&mut self) -> Vec<u8> {
        self.url("/api/v0/bootstrap/list").query()
    }

    pub fn bootstrap_rm_all(&mut self) -> Vec<u8> {
        self.url("/api/v0/bootstrap/rm/all").query()
    }

    pub fn cat(&mut self, hash: &str) -> Vec<u8> {
        let l = format!("arg={}", hash);
        self.url("/api/v0/cat").args(&l).query()
    }

    pub fn commands(&mut self) -> Vec<u8> {
        self.url("/api/v0/commands").query()
    }

    pub fn config_replace(&mut self, raw_path: &str) -> Vec<u8> {
        self.url("/api/v0/config/replace").path(raw_path).query_post_upload()
    }

    pub fn config_show(&mut self) -> Vec<u8> {
        self.url("/api/v0/config/show").query()
    }

    pub fn dag_get(&mut self, inp: &str) -> Vec<u8> {
        let l = format!("arg={}", inp);
        self.url("/api/v0/dag/get").args(&l).query()
    }

    pub fn dag_put(&mut self, raw_path: &str, fmt: &str, ienc: &str) -> Vec<u8> {
        let l = format!("format={}&input-enc={}", fmt, ienc);
        self.url("/api/v0/dag/put").args(&l).path(raw_path).query_post_upload()
    }

    pub fn dht_findpeer(&mut self, peer: &str, verbose: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        let l = format!("arg={}&verbose={}", peer, v);
        self.url("/api/v0/dht/findpeer").args(&l).query()
    }

    pub fn dht_findprovs(&mut self, key: &str, verbose: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        let l = format!("arg={}&verbose={}", key, v);
        self.url("/api/v0/dht/findprovs").args(&l).query()
    }

    pub fn dht_get(&mut self, key: &str, verbose: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        let l = format!("arg={}&verbose={}", key, v);
        self.url("/api/v0/dht/get").args(&l).query()
    }

    pub fn dht_provide(&mut self, key: &str, verbose: bool, recursive: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        let r = &parse_bool(recursive);
        let l = format!("arg={}&verbose={}&recursive={}", key, v, r);
        self.url("/api/v0/dht/provide").args(&l).query()
    }

    pub fn dht_put(&mut self, key: &str, val: &str, verbose: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        // key goes first, then goes value and it will be stored as a dict mem.
        let l = format!("arg={}&arg={}&verbose={}", key, val, v);
        self.url("/api/v0/dht/put").args(&l).query()
    }

    pub fn dht_query(&mut self, peer: &str, verbose: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        let l = format!("arg={}&verbose={}", peer, v);
        self.url("/api/v0/dht/query").args(&l).query()
    }

    pub fn diag_cmds_clear(&mut self) -> Vec<u8> {
        self.url("/api/v0/diag/cmds/clear").query()
    }

    pub fn diag_set_time(&mut self, ntime: &str) -> Vec<u8> {
        let l = format!("arg={}", ntime);
        self.url("/api/v0/diag/cmds/set-time").args(&l).query()
    }

    pub fn diag_net(&mut self, vis: &str) -> Vec<u8> {
        let l = format!("vis={}", vis);
        self.url("/api/v0/diag/net").args(&l).query()
    }

    pub fn diag_sys(&mut self) -> Vec<u8> {
        self.url("/api/v0/diag/sys").query()
    }

    pub fn dns(&mut self, link: &str, recursive: bool) -> Vec<u8> {
        let r = &parse_bool(recursive);
        let l = format!("arg={}&recursive={}", link, r);
        self.url("/api/v0/dns").args(&l).query()
    }

    pub fn file_ls(&mut self, ipath: &str) -> Vec<u8> {
        let l = format!("arg={}", ipath);
        self.url("/api/v0/file/ls").args(&l).query()
    }

    pub fn files_cp(&mut self, from: &str, to: &str) -> Vec<u8> {
        let l = format!("arg={}&arg={}", from, to);
        self.url("/api/v0/files/cp").args(&l).query()
    }

    pub fn files_flush(&mut self, ipath: &str) -> Vec<u8> {
        let l = format!("arg={}", ipath);
        self.url("/api/v0/files/flush").args(&l).query()
    }

    pub fn files_ls(&mut self, ipath: &str, ilong: bool) -> Vec<u8> {
        let long = &parse_bool(ilong);
        let l = format!("arg={}&long={}", ipath, long);
        self.url("/api/v0/files/flush").args(&l).query()
    }

    pub fn files_mkdir(&mut self, ipath: &str, prnts: bool) -> Vec<u8> {
        let p = &parse_bool(prnts);
        let l = format!("arg={}&parents={}", ipath, p);
        self.url("/api/v0/files/mkdir").args(&l).query()
    }

    pub fn files_mv(&mut self, source: &str, dest: &str) -> Vec<u8> {
        let l = format!("arg={}&arg={}", source, dest);
        self.url("/api/v0/files/mv").args(&l).query()
    }

    pub fn files_read(&mut self, ipath: &str, off: i64, count: i64) -> Vec<u8> {
        let l = format!("arg={}&offset={}&count={}", ipath, &off.to_string(), &count.to_string());
        self.url("/api/v0/files/read").args(&l).query()
    }

    pub fn files_rm(&mut self, ipath: &str, recursive: bool) -> Vec<u8> {
        let r = &parse_bool(recursive);
        let l = format!("arg={}&recursive={}", ipath, r);
        self.url("/api/v0/files/rm").args(&l).query()
    }

    pub fn files_stat(&mut self, ipath: &str, fmt: &str, hash: bool, size: bool) -> Vec<u8> {
        let h = &parse_bool(hash);
        let s = &parse_bool(size);
        let l = format!("arg={}&format={}&hash={}&size={}", ipath, fmt, h, s);
        self.url("/api/v0/files/stat").args(&l).query()
    }

    pub fn files_write(&mut self, ipath: &str, raw_path: &str, off: i64,
                        create: bool, truncate: bool, count: i64) -> Vec<u8> {
        let c = &parse_bool(create);
        let t = &parse_bool(truncate);
        let l = format!("arg={}&arg={}&offset={}&create={}&truncate={}&count={}",
                        ipath, raw_path, &off.to_string(), c, t, &count.to_string());
        self.url("/api/v0/files/write").args(&l).query_post_upload()
    }

    pub fn filestore_dups(&mut self) -> Vec<u8> {
        self.url("/api/v0/filestore/dups").query()
    }

    pub fn filestore_ls(&mut self, cid: &str) -> Vec<u8> {
        let l = format!("arg={}", cid);
        self.url("/api/v0/filestore/ls").args(&l).query()
    }

    pub fn filestore_verify(&mut self, cid: &str) -> Vec<u8> {
        let l = format!("arg={}", cid);
        self.url("/api/v0/filestore/verify").args(&l).query()
    }

    pub fn get(&mut self, ipath: &str,
                raw_path: &str, archive: bool,
                compress: bool, clevel: i8) -> Vec<u8> {
        let a = &parse_bool(archive);
        let c = &parse_bool(compress);
        let l = format!("arg={}&arg={}&archive={}&compress={}&compression-level={}",
                        ipath, raw_path, a, c, &clevel.to_string());
        self.url("/api/v0/get").args(&l).query()
    }

    pub fn id(&mut self, peer: &str, fmt: &str) -> Vec<u8> {
        let l = format!("arg={}&format={}", peer, fmt);
        self.url("/api/v0/id").args(&l).query()
    }

    pub fn key_gen(&mut self, name: &str, itype: &str, size: i64) -> Vec<u8> {
        let l = format!("arg={}&type={}&size={}", name, itype, &size.to_string());
        self.url("/api/v0/key/gen").args(&l).query()
    }

    pub fn key_list(&mut self, extra: bool) -> Vec<u8> {
        let e = &parse_bool(extra);
        let l = format!("l={}", e);
        self.url("/api/v0/key/list").args(&l).query()
    }

    pub fn log_level(&mut self, sli: &str, level: &str) -> Vec<u8> {
        let l = format!("arg={}&arg={}", sli, level);
        self.url("/api/v0/log/level").args(&l).query()
    }

    pub fn log_ls(&mut self) -> Vec<u8> {
        self.url("/api/v0/log/ls").query()
    }

    pub fn log_tail(&mut self) -> Vec<u8> {
        self.url("/api/v0/log/tail").query()
    }

    pub fn ls(&mut self, ipath: &str, hdrs: bool, rtype: bool) -> Vec<u8> {
        let h = &parse_bool(hdrs);
        let r = &parse_bool(rtype);
        let l = format!("arg={}&headers={}&resolve-type={}", ipath, h, r);
        self.url("/api/v0/ls").args(&l).query()
    }

    pub fn mount(&mut self, ipfs_path: &str, ipns_path: &str) -> Vec<u8> {
        let l = format!("ipfs-path={}&ipns-path={}", ipfs_path, ipns_path);
        self.url("/api/v0/mount").args(&l).query()
    }

    pub fn name_publish(&mut self, ipath: &str,
                        resolve: bool, lifetime: &str,
                        ttl: &str, key: &str) -> Vec<u8> {
        let r = &parse_bool(resolve);
        let l = format!("arg={}&resolve={}&lifetime={}&ttl={}&key={}",
                ipath, r, lifetime, ttl, key);
        self.url("/api/v0/name/publish").args(&l).query()
    }

    pub fn name_resolve(&mut self, iname: &str, recursive: bool, nocache: bool) -> Vec<u8> {
        let r = &parse_bool(recursive);
        let n = &parse_bool(nocache);
        let l = format!("arg={}&recursive={}&nocache={}", iname, r, n);
        self.url("/api/v0/name/resolve").args(&l).query()
    }

    pub fn object_data(&mut self, key: &str) -> Vec<u8> {
        let l = format!("arg={}", key);
        self.url("/api/v0/object/data").args(&l).query()
    }

    pub fn object_diff(&mut self, left: &str, right: &str, verbose: bool) -> Vec<u8> {
        let v = &parse_bool(verbose);
        let l = format!("arg={}&arg={}&verbose={}", left, right, v);
        self.url("/api/v0/object/diff").args(&l).query()
    }

    pub fn object_get(&mut self, key: &str) -> Vec<u8> {
        let l = format!("arg={}", key);
        self.url("/api/v0/object/get").args(&l).query()
    }

    pub fn object_links(&mut self, key: &str, hdrs: bool) -> Vec<u8> {
        let h = &parse_bool(hdrs);
        let l = format!("arg={}&headers={}", key, h);
        self.url("/api/v0/object/links").args(&l).query()
    }

    pub fn object_new(&mut self, obj: &str) -> Vec<u8> {
        let l = format!("arg={}", obj);
        self.url("/api/v0/object/new").args(&l).query()
    }

    pub fn object_patch_add_link(&mut self, hash: &str, iname: &str, iobj: &str, create: bool) -> Vec<u8> {
        let c = &parse_bool(create);
        let l = format!("arg={}&arg={}&arg={}&create={}", hash, iname, iobj, c);
        self.url("/api/v0/object/patch/add-link").args(&l).query()
    }

    pub fn object_patch_append_data(&mut self, hash: &str, raw_path: &str) -> Vec<u8> {
        let l = format!("arg={}", hash);
        self.url("/api/v0/object/patch/append-data").args(&l).path(raw_path).query_post_upload()
    }

    pub fn object_patch_rm_link(&mut self, hash: &str, iname: &str) -> Vec<u8> {
        let l = format!("arg={}&arg={}", hash, iname);
        self.url("/api/v0/object/patch/rm-link").args(&l).query()
    }

    pub fn object_patch_set_data(&mut self, hash: &str, raw_path: &str) -> Vec<u8> {
        let l = format!("arg={}", hash);
        self.url("/api/v0/object/patch/set-data").args(&l).path(raw_path).query_post_upload()
    }

    pub fn object_put(&mut self, raw_path: &str, ienc: &str, dfenc: &str) -> Vec<u8> {
        let l = format!("inputenc={}&datafieldenc={}", ienc, dfenc);
        self.url("/api/v0/object/put").args(&l).path(raw_path).query_post_upload()
    }

    pub fn object_stat(&mut self, key: &str) -> Vec<u8> {
        let l = format!("arg={}", key);
        self.url("/api/v0/object/stat").args(&l).query()
    }

    pub fn pin_add(&mut self, ipath: &str, recursive: bool, progress: bool) -> Vec<u8> {
        let r = &parse_bool(recursive);
        let p = &parse_bool(progress);
        let l = format!("arg={}&recursive={}&progress={}", ipath, r, p);
        self.url("/api/v0/pin/add").args(&l).query()
    }

    pub fn pin_ls(&mut self, ipath: &str, itype: &str, quiet: bool) -> Vec<u8> {
        let q = &parse_bool(quiet);
        let l = format!("arg={}&type={}&quiet={}", ipath, itype, q);
        self.url("/api/v0/pin/ls").args(&l).query()
    }

    pub fn pin_rm(&mut self, ipath: &str, recursive: bool) -> Vec<u8> {
        let r = &parse_bool(recursive);
        let l = format!("arg={}&recursive={}", ipath, r);
        self.url("/api/v0/pin/rm").args(&l).query()
    }

    pub fn ping(&mut self, peer: &str, count: i32) -> Vec<u8> {
        let l = format!("arg={}&count={}", peer, &count.to_string());
        self.url("/api/v0/ping").args(&l).query()
    }

    pub fn pubsub_ls(&mut self) -> Vec<u8> {
        self.url("/api/v0/pubsub/ls").query()
    }

    pub fn pubsub_peers(&mut self, topic: &str) -> Vec<u8> {
        let l = format!("arg={}", topic);
        self.url("/api/v0/pubsub/peers").args(&l).query()
    }

    pub fn pubsub_pub(&mut self, topic: &str, payload: &str) -> Vec<u8> {
        let l = format!("arg={}&payload={}", topic, payload);
        self.url("/api/v0/pubsub/pub").args(&l).query()
    }

    pub fn pubsub_sub(&mut self, topic: &str, discover: bool) -> Vec<u8> {
        let d = &parse_bool(discover);
        let l = format!("arg={}&discover={}", topic, d);
        self.url("/api/v0/pubsub/sub").args(&l).query()
    }

    pub fn refs_local(&mut self) -> Vec<u8> {
        self.url("/api/v0/refs/local").query()
    }

    pub fn repo_fsck(&mut self) -> Vec<u8> {
        self.url("/api/v0/repo/fsck").query()
    }

    pub fn repo_gc(&mut self, quiet: bool, serr: bool) -> Vec<u8> {
        let q = &parse_bool(quiet);
        let s = &parse_bool(serr);
        let l = format!("quiet={}&stream-errors={}", q, s);
        self.url("/api/v0/repo/gc").args(&l).query()
    }

    pub fn repo_stat(&mut self, human: bool) -> Vec<u8> {
        let h = &parse_bool(human);
        let l = format!("human={}", h);
        self.url("/api/v0/repo/stat").args(&l).query()
    }

    pub fn repo_verify(&mut self) -> Vec<u8> {
        self.url("/api/v0/repo/verify").query()
    }

    pub fn repo_version(&mut self, quiet: bool) -> Vec<u8> {
        let q = &parse_bool(quiet);
        let l = format!("quiet={}", q);
        self.url("/api/v0/repo/version").args(&l).query()
    }

    pub fn resolve(&mut self, iname: &str, recursive: bool) -> Vec<u8> {
        let r = &parse_bool(recursive);
        let l = format!("arg={}&recursive={}", iname, r);
        self.url("/api/v0/resolve").args(&l).query()
    }

    pub fn stats_bitswap(&mut self) -> Vec<u8> {
        self.url("/api/v0/stats/bitswap").query()
    }

    pub fn stats_bw(&mut self, peer: &str, proto: &str, poll: bool, interval: &str) -> Vec<u8> {
        let p = &parse_bool(poll);
        let l = format!("peer={}&proto={}&poll={}&interval={}", peer, proto, p, interval);
        self.url("/api/v0/stats/bw").args(&l).query()
    }

    pub fn stats_repo(&mut self, human: bool) -> Vec<u8> {
        let h = &parse_bool(human);
        let l = format!("human={}", h);
        self.url("/api/v0/stats/repo").args(&l).query()
    }

    pub fn swarm_addrs_local(&mut self, peer: &str) -> Vec<u8> {
        let l = format!("arg={}", peer);
        self.url("/api/v0/swarm/addrs/local").args(&l).query()
    }

    pub fn swarm_connect(&mut self, addrs: &str) -> Vec<u8> {
        let l = format!("arg={}", addrs);
        self.url("/api/v0/swarm/connect").args(&l).query()
    }

    pub fn swarm_disconnect(&mut self, addrs: &str) -> Vec<u8> {
        let l = format!("arg={}", addrs);
        self.url("/api/v0/swarm/disconnect").args(&l).query()
    }

    pub fn swarm_filters_add(&mut self, maddr: &str) -> Vec<u8> {
        let l = format!("arg={}", maddr);
        self.url("/api/v0/swarm/filters/add").args(&l).query()
    }

    pub fn swarm_filderst_rm(&mut self, maddr: &str) -> Vec<u8> {
        let l = format!("arg={}", maddr);
        self.url("/api/v0/swarm/filters/rm").args(&l).query()
    }

    pub fn swarm_peers(&mut self) -> Vec<u8> {
        self.url("/api/v0/swarm/peers").query()
    }

    pub fn tar_add(&mut self, raw_path: &str) -> Vec<u8> {
        self.url("/api/v0/tar/add").path(raw_path).query_post_upload()
    }

    pub fn tar_cat(&mut self, ipath: &str) -> Vec<u8> {
        let l = format!("arg={}", ipath);
        self.url("/api/v0/tar/cat").args(&l).query()
    }

    pub fn tour_list(&mut self) -> Vec<u8> {
        self.url("/api/v0/tour/list").query()
    }

    pub fn tour_next(&mut self) -> Vec<u8> {
        self.url("/api/v0/tour/next").query()
    }

    pub fn tour_restart(&mut self) -> Vec<u8> {
        self.url("/api/v0/tour/restart").query()
    }

    pub fn update(&mut self, sargs: &str) -> Vec<u8> {
        let l = format!("arg={}", sargs);
        self.url("/api/v0/update").args(&l).query()
    }

    pub fn version(&mut self, num: bool, com: bool, repo: bool, all: bool) -> Vec<u8> {
        let n = &parse_bool(num);
        let c = &parse_bool(com);
        let r = &parse_bool(repo);
        let a = &parse_bool(all);
        let l = format!("number={}&commit={}&repo={}&all={}", n, c, r, a);
        self.url("/api/v0/version").args(&l).query()
    }


    //------------------------------------------- QUERIES -----------------------------------------


    fn complete_post_link(&self) -> String {
        let link = if self.url.chars().nth(0).unwrap() == '/' {
            format!("{}", &self.url)
        } else {
            format!("/{}", &self.url)
        };
        self.host.to_string() + ":" + &self.port.to_string() + &link
    }

    fn complete_get_link(&self) -> String {
        let l = self.complete_post_link();
        if !&self.args.is_empty() {
            return format!("{}?{}", l, &self.args)
        }
        return l
    }

    fn query(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut handle = Easy::new();
        let l = &self.complete_get_link();
        handle.url(l).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        data
    }

    fn query_post_upload(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut handle = Easy::new();
        let mut frm    = Form::new();
        let path       = Path::new(&self.path);
        frm.part("arg").file(path).add().unwrap();
        handle.url(&self.complete_get_link()).unwrap();
        handle.httppost(frm).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|new_data| {
                buffer.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        buffer
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use coder::Coder;

    #[test]
    fn cat_returns_correct_value() {
        let mut ipfs = IPFS::new();
        ipfs.host("http://localhost", 5001);
        let ipfs_response = ipfs.cat("QmaGXbCcuNazWyCmdiHsN9bdZ1GEx1GArUvbmyzkHmotDH");
        let parsed_response = Coder::decode_to_str(ipfs_response);
        assert_eq!("hello, it really works!\n", parsed_response);
    }

    #[test]
    fn add_returns_correct_hash() {
        let mut ipfs = IPFS::new();
        ipfs.host("http://localhost", 5001);
        let ipfs_response = ipfs.add("./it_works.txt");
        let parsed_response = Coder::to_json2(&ipfs_response);
        let hashsumm = &parsed_response["Hash"].to_string();
        assert_eq!("QmaGXbCcuNazWyCmdiHsN9bdZ1GEx1GArUvbmyzkHmotDH", hashsumm);
    }

    #[test]
    fn version_returns_correct_ver() {
        let mut ipfs = IPFS::new();
        ipfs.host("http://localhost", 5001);
        let ipfs_response = ipfs.version(false, false, false, false);
        let parsed_response = Coder::decode_to_str(ipfs_response);
        assert_eq!("{\"Version\":\"0.4.10\",\"Commit\":\"4679f80\",\"Repo\":\"5\",\"System\":\"amd64/darwin\",\"Golang\":\"go1.8.3\"}\n", parsed_response);
    }

    #[test]
    fn pubsub_ls_returns_warning() {
        let mut ipfs = IPFS::new();
        ipfs.host("http://localhost", 5001);
        let ipfs_response = ipfs.pubsub_ls();
        let parsed_response = Coder::decode_to_str(ipfs_response);
        assert_eq!("{\"Message\":\"experimental pubsub feature not enabled. Run daemon with --enable-pubsub-experiment to use.\",\"Code\":0}\n", parsed_response);
    }
}
