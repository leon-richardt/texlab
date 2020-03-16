fn main() {
    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .flag_if_supported("-w")
        .define("_CRT_SECURE_NO_WARNINGS", None)
        .file("vendor/adsout.c")
        .file("vendor/adsout_journals.c")
        .file("vendor/bibcore.c")
        .file("vendor/bibl.c")
        .file("vendor/biblatexin.c")
        .file("vendor/bibtexin.c")
        .file("vendor/bibtexout.c")
        .file("vendor/bibtextypes.c")
        .file("vendor/bibutils.c")
        .file("vendor/bltypes.c")
        .file("vendor/bu_auth.c")
        .file("vendor/charsets.c")
        .file("vendor/copacin.c")
        .file("vendor/copactypes.c")
        .file("vendor/ebiin.c")
        .file("vendor/endin.c")
        .file("vendor/endout.c")
        .file("vendor/endtypes.c")
        .file("vendor/endxmlin.c")
        .file("vendor/entities.c")
        .file("vendor/fields.c")
        .file("vendor/gb18030.c")
        .file("vendor/generic.c")
        .file("vendor/intlist.c")
        .file("vendor/isiin.c")
        .file("vendor/isiout.c")
        .file("vendor/isitypes.c")
        .file("vendor/iso639_1.c")
        .file("vendor/iso639_2.c")
        .file("vendor/iso639_3.c")
        .file("vendor/is_ws.c")
        .file("vendor/latex.c")
        .file("vendor/marc_auth.c")
        .file("vendor/medin.c")
        .file("vendor/modsin.c")
        .file("vendor/modsout.c")
        .file("vendor/modstypes.c")
        .file("vendor/name.c")
        .file("vendor/nbibin.c")
        .file("vendor/nbibout.c")
        .file("vendor/nbibtypes.c")
        .file("vendor/notes.c")
        .file("vendor/pages.c")
        .file("vendor/reftypes.c")
        .file("vendor/risin.c")
        .file("vendor/risout.c")
        .file("vendor/ristypes.c")
        .file("vendor/serialno.c")
        .file("vendor/slist.c")
        .file("vendor/str.c")
        .file("vendor/strsearch.c")
        .file("vendor/str_conv.c")
        .file("vendor/title.c")
        .file("vendor/type.c")
        .file("vendor/unicode.c")
        .file("vendor/url.c")
        .file("vendor/utf8.c")
        .file("vendor/vplist.c")
        .file("vendor/wordin.c")
        .file("vendor/wordout.c")
        .file("vendor/xml.c")
        .file("vendor/xml_encoding.c")
        .compile("bibutils");
}
