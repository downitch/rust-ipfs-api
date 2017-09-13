# rust-ipfs-api

![](https://ipfs.io/ipfs/QmQJ68PFMDdAsgCZvA1UVzzn18asVcf7HVvCDgpjiSCAse)

> A Rust client library for the IPFS HTTP API


# Rust inspires thousands people to create and develop many services
Assuming this, IPFS is one of the most popular technologies to use to, but there is no easy way to use IPFS API in projects on rust. People have to run Go programs, or create cURL/reqwest/hyper wrappers in order to use IPFS cURL API. Having this repo as a dependency it is mush easier to start developing your apps on top of IPFS! 
<hr>
Let's say, you want to store files in IPFS and read them right from the net:

    mod coder;
    mod ipfs-api;
    
    use ipfs-api::IPFS;
    use coder::Coder;
    
    fn main() {
        let mut ipfs = IPFS::new();
        ipfs.host("http://localhost", 5001);
    
        let ipfs_response = ipfs.add("./it_works.txt");
        let parsed_response = Coder::to_json2(&ipfs_response);
        let hashsumm = &parsed_response["Hash"].to_string();
        assert_eq!("QmaGXbCcuNazWyCmdiHsN9bdZ1GEx1GArUvbmyzkHmotDH", &hashsumm); // TRUE
    
        let ipfs_response = ipfs.cat(hashsumm);
        let parsed_response = Coder::decode_to_str(ipfs_response);
        assert_eq!("hello, it really works!\n", parsed_response); // TRUE
    }
 
 Easy, huh? That's the most common way to use API :)
 Try it out and don't forget to help the project!
