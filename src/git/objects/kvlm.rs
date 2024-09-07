use ordermap::OrderMap;

pub fn kvlm_parse(
    raw: &[u8],
    start: usize,
    mut dct: Option<OrderMap<Option<Vec<u8>>, Vec<u8>>>,
) -> OrderMap<Option<Vec<u8>>, Vec<u8>> {
    if dct.is_none() {
        dct = Some(OrderMap::new());
    }
    let mut dct = dct.unwrap();

    let spc = raw[start..]
        .iter()
        .position(|&c| c == b' ')
        .map(|p| p + start);
    let nl = raw[start..]
        .iter()
        .position(|&c| c == b'\n')
        .map(|p| p + start);

    if spc.is_none() || (nl.is_some() && nl.unwrap() < spc.unwrap()) {
        assert_eq!(nl.unwrap(), start);
        dct.insert(None, raw[start + 1..].to_vec());
        return dct;
    }

    let spc = spc.unwrap();
    let key = &raw[start..spc];

    let mut end = start;
    loop {
        end = raw[end + 1..]
            .iter()
            .position(|&c| c == b'\n')
            .map(|p| p + end + 1)
            .unwrap();
        if raw[end + 1] != b' ' {
            break;
        }
    }

    let mut value = Vec::new();
    let value_slice = &raw[spc + 1..end];

    let mut i = 0;
    while i < value_slice.len() {
        if i + 1 < value_slice.len() && value_slice[i] == b'\n' && value_slice[i + 1] == b' ' {
            value.push(b'\n');
            i += 2;
        } else {
            value.push(value_slice[i]);
            i += 1;
        }
    }

    if let Some(existing_value) = dct.get_mut(&Some(key.to_vec())) {
        if existing_value.is_empty() {
            *existing_value = value;
        } else {
            existing_value.extend(value);
        }
    } else {
        dct.insert(Some(key.to_vec()), value);
    }

    kvlm_parse(raw, end + 1, Some(dct))
}

pub fn kvlm_serialize(kvlm: OrderMap<Option<Vec<u8>>, Vec<u8>>) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    // rust code:
    for key in kvlm.keys() {
        if key.is_none() {
            continue;
        }
        let val = kvlm.get(key).unwrap();
        let val = val.clone();

        for v in val.iter() {
            ret.extend(key.clone().unwrap());
            ret.push(b' ');
            if *v == b'\n' {
                ret.extend(b"\n ");
            }
            ret.push(b'\n');
        }
    }

    // Append the message
    ret.push(b'\n');
    ret.extend(kvlm.get(&None).unwrap());
    ret.push(b'\n');

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvlm_parse() {
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

Create first draft";

        let map = kvlm_parse(raw, 0, None);

        assert_eq!(map.len(), 6);
        assert_eq!(
            map.get(&Some(b"tree".to_vec())).unwrap(),
            b"29ff16c9c14e2652b22f8b78bb08a5a07930c147"
        );
        assert_eq!(
            map.get(&Some(b"parent".to_vec())).unwrap(),
            b"206941306e8a8af65b66eaaaea388a7ae24d49a0"
        );
        assert_eq!(
            map.get(&Some(b"author".to_vec())).unwrap(),
            b"Thibault Polge <thibault@thb.lt> 1527025023 +0200"
        );
        assert_eq!(
            map.get(&Some(b"committer".to_vec())).unwrap(),
            b"Thibault Polge <thibault@thb.lt> 1527025044 +0200"
        );
        assert_eq!(
            map.get(&Some(b"gpgsig".to_vec())).unwrap(),
            b"-----BEGIN PGP SIGNATURE-----"
        );
        assert_eq!(
            map.get(&None).unwrap(),
            b" iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL
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

Create first draft"
        );
    }
}
