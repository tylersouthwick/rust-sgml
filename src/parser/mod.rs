use io::Read;

fn parse(content : Read) {
}

mod test {

    use super::parse;

    #[test]
    fn it_works2() {
        let test ="<OFX><SIGNONMSGSRSV1><SONRS><STATUS><CODE>15500<SEVERITY>ERROR</STATUS><DTSERVER>20150510024536.630<LANGUAGE>ENG<DTACCTUP>20150510024536.630<FI><ORG>ING DIRECT<FID>031176110</FI></SONRS></SIGNONMSGSRSV1></OFX>";
        parse(test);
    }
}
