use crate::utils;

//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[test]
fn passing_decode_unicode_characters() {
    assert_eq!(
        utils::decode_url(str!(
            "%E6%A4%9C%E3%83%92%E3%83%A0%E8%A7%A3%E5%A1%97%E3%82%83%E3%83%83%20%3D%20%E3%82%B5"
        )),
        "検ヒム解塗ゃッ = サ"
    );
}

#[test]
fn passing_decode_file_url() {
    assert_eq!(
        utils::decode_url(str!("file:///tmp/space%20here/test%231.html")),
        "file:///tmp/space here/test#1.html"
    );
}

#[test]
fn passing_plus_sign() {
    assert_eq!(
        utils::decode_url(str!(
            "fonts.somewhere.com/css?family=Open+Sans:300,400,400italic,600,600italic"
        )),
        "fonts.somewhere.com/css?family=Open+Sans:300,400,400italic,600,600italic"
    );
}
