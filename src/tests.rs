
#![cfg(test)]

mod tests {

    use crypto::sha3::Sha3;

    use MerkleTree;
    use MerkleDigest;
    use Hashable;

    #[test]
    fn test_from_vec3() {
        let mut digest = Sha3::sha3_256();

        let values = vec![1, 2, 3];
        let hashes = vec![digest.hash_bytes(&1.to_bytes()), digest.hash_bytes(&2.to_bytes()), digest.hash_bytes(&3.to_bytes())];

        let tree   = MerkleTree::from_vec(digest, values);

        let root_hash = Sha3::sha3_256().combine_hashes(
            &Sha3::sha3_256().combine_hashes(&hashes[0], &hashes[1]),
            &Sha3::sha3_256().hash_bytes(&hashes[2])
        );

        assert_eq!(tree.count, 3);
        assert_eq!(tree.height, 2);
        assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
    }

    #[test]
    fn test_from_vec9() {
        let mut digest = Sha3::sha3_256();

        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let hashes = values.clone().iter().map(|v| digest.hash_bytes(&v.to_bytes())).collect::<Vec<_>>();
        let count  = values.len();

        let tree   = MerkleTree::from_vec(digest, values);

        let root_hash = Sha3::sha3_256().combine_hashes(
            &Sha3::sha3_256().combine_hashes(
                &Sha3::sha3_256().combine_hashes(
                    &Sha3::sha3_256().combine_hashes(&hashes[0], &hashes[1]),
                    &Sha3::sha3_256().combine_hashes(&hashes[2], &hashes[3])
                ),
                &Sha3::sha3_256().combine_hashes(
                    &Sha3::sha3_256().combine_hashes(&hashes[4], &hashes[5]),
                    &Sha3::sha3_256().combine_hashes(&hashes[6], &hashes[7])
                )
            ),
            &Sha3::sha3_256().hash_bytes(&hashes[8])
        );

        assert_eq!(tree.count, count);
        assert_eq!(tree.height, 4);
        assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
    }

}

