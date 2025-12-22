use crate::lang_api::LanguageApi;

mod lang_api;
mod models;

static ISO_639_1_LANGUAGES: &[&str] = &[
    "aa","ab","af","ak","sq","am","ar","an","hy","as","av","ay","az",
    "ba","bm","eu","be","bn","bh","bi","bs","br","bg","my","ca","ch","ce","zh",
    "cv","kw","co","cr","cy","da","de","dv","dz","el","en","et","ee",
    "fo","fa","fj","fi","fr","fy","ff","gd","ga","gl","gv","gn","gu","ht","ha",
    "he","hz","hi","ho","hr","hu","ig","is","ii","iu","ia","id","ik",
    "it","jv","ja","kl","kn","ks","ka","kr","kk","km","ki","rw","ky","kv","kg",
    "ko","kj","ku","lo","la","lv","li","ln","lt","lb","lu","lg","mk","mh","ml",
    "mi","mr","ms","mg","mt","mn","na","nv","nr","nd","ng","ne","nl","nn","nb",
    "no","ny","oc","oj","or","om","os","pa","pi","pl","pt","ps","qu","rm","ro",
    "rn","ru","sg","sa","si","sk","sl","se","sm","sn","sd","so","st","es","sc",
    "sr","ss","su","sw","sv","ty","ta","tt","te","tg","tl","th","bo","ti","to",
    "tn","ts","tk","tr","tw","ug","uk","ur","uz","ve","vi","wa","wo","xh",
    "yi","za","zu"
];

#[tokio::main]
async fn main() {
    let client = LanguageApi::new("http://localhost:3000".into());

    for iso639p1 in ISO_639_1_LANGUAGES {
        let result = client.get_language_metadata(iso639p1).await.unwrap();

        if result.is_none() {
            println!("{}", iso639p1);
        }
    }

    // dbg!(result);
}
