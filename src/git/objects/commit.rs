use crate::git::objects::objects::GitObject;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct GitCommit {
    pub fmt: Vec<u8>,
    pub data: Vec<u8>,
}

impl GitObject for GitCommit {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }
}

type OrderedMap = BTreeMap<Option<Vec<u8>>, Vec<Vec<u8>>>;

fn kvlm_parse(raw: Vec<u8>, start: usize, map: Option<OrderedMap>) -> OrderedMap {
    let mut map: OrderedMap = match map {
        Some(m) => m,
        None => BTreeMap::new(),
    };

    dbg!(String::from_utf8(raw[start..].to_vec()));

    let spc = raw[start..].iter().position(|&b| b == b' ');
    let nl = raw[start..]
        .iter()
        .position(|&b| b == b'\n')
        .expect("Invalid object");

    if spc.is_none() || (nl < spc.unwrap()) {
        assert_eq!(nl, start);
        map.insert(None, vec![raw[start + 1..].to_vec()]);
        return map;
    }
    let spc = spc.unwrap();
    let key = raw[start..spc].to_vec();
    let mut end = start;

    loop {
        end = raw[end + 1..].iter().position(|&b| b == b'\n').unwrap();
        if raw[end + 1] != b' ' {
            break;
        }
    }

    let slice = &raw[spc + 1..end];
    let mut value = Vec::with_capacity(slice.len());

    let mut i = 0;
    while i < slice.len() {
        if slice[i..].starts_with(b"\n ") {
            value.push(b'\n');
            i += 2; // Skip over "\n "
        } else {
            value.push(slice[i]);
            i += 1;
        }
    }

    if map.contains_key(&Some(key.clone())) {
        if let Some(existing_value) = map.get_mut(&Some(key.clone())) {
            existing_value.push(value);
        } else {
            map.insert(Some(key.clone()), vec![value]);
        }
    } else {
        map.insert(Some(key.clone()), vec![value[..].to_vec()]);
    }
    // print map
    for (k, v) in map.iter() {
        println!("{:?} {:?}", k, v);
    }

    kvlm_parse(raw, end + 1, Some(map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvlm_parse() {
        let raw1 = b"hi there
";
        let raw2 = b"author Thibault Polge <thibault@thb.lt> 1527025023 +0200
";
        let raw = b"tree 29ff16c9c14e2652b22f8b78bb08a5a07930c147
parent 206941306e8a8af65b66eaaaea388a7ae24d49a0
author Thibault Polge <thibault@thb.lt> 1527025023 +0200
committer Thibault Polge <thibault@thb.lt> 1527025044 +0200
gpgsig -----BEGIN PGP SIGNATURE-----

 iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL
 kGQdcBAAqPP+ln4nGDd2gETXjvOpOxLzIMEw4A9gU6CzWzm+oB8mEIKyaH0UFIPh
 rNUZ1j7/ZGFNeBDtT55LPdPIQw4KKlcf6kC8MPWP3qSu3xHqx12C5zyai2duFZUU
 wqOt9iCFCscFQYqKs3xsHI+ncQb+PGjVZA8+jPw7nrPIkeSXQV2aZb1E68wa2YIL
 3eYgTUKz34cB6tAq9YwHnZpyPx8UJCZGkshpJmgtZ3mCbtQaO17LoihnqPn4UOMr
 V75R/7FjSuPLS8NaZF4wfi52btXMSxO/u7GuoJkzJscP3p4qtwe6Rl9dc1XC8P7k
 NIbGZ5Yg5cEPcfmhgXFOhQZkD0yxcJqBUcoFpnp2vu5XJl2E5I/quIyVxUXi6O6c
 /obspcvace4wy8uO0bdVhc4nJ+Rla4InVSJaUaBeiHTW8kReSFYyMmDCzLjGIu1q
 doU61OM3Zv1ptsLu3gUE6GU27iWYj2RWN3e3HE4Sbd89IFwLXNdSuM0ifDLZk7AQ
 WBhRhipCCgZhkj9g2NEk7jRVslti1NdN5zoQLaJNqSwO1MtxTmJ15Ksk3QP6kfLB
 Q52UWybBzpaP9HEd4XnR+HuQ4k2K0ns2KgNImsNvIyFwbpMUyUWLMPimaV1DWUXo
 5SBjDB/V/W2JBFR+XKHFJeFwYhj7DD/ocsGr4ZMx/lgc8rjIBkI=
 =lgTX
 -----END PGP SIGNATURE-----

Create first draft
";

        let map = kvlm_parse(raw.to_vec(), 0, None);
    }
}
