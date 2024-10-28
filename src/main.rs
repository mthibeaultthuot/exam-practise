use axum::{
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_question)).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Question {
    pub question: &'static str,
    pub answer: &'static str,
}

impl Question {
    pub fn new(question: &'static str, answer: &'static str) -> Self {
        Self { question, answer }
    }
}

async fn get_question() -> impl IntoResponse {
    let mut quiz = Vec::new();
    quiz.push(Question::new(
        "Question3 (vrai/faux) - Marx est un philosophe du 18e siècle, le siècle des Lumières",
        "Faux",
    ));
    quiz.push(Question::new(
        "Question4 (vrai/faux) - Marx est un philosophe humaniste, un héritier des promesses humanistes de la Révolution française",
        "Vrai",
    ));
    quiz.push(Question::new(
        "Question5 (vrai/faux)- La capitalisme (que Marx critique) apparaît au 20e siècle, après la Deuxième Guerre mondiale, dans la Guerre froide entre le capitalisme américain (bloc de l’Ouest) et le communisme soviétique (le bloc de l’Est)",
        "Faux",
    ));
    quiz.push(Question::new("Question6 (vrai/faux) - Marx ne propose pas une explication du capitalisme, mais une critique du capitalisme", "Faux"));
    quiz.push(Question::new("Question7 (vrai/faux) - Marx est un communiste, un égalitariste: il est donc un anticapitaliste et, à ce titre, veut interdire la liberté d’entreprendre, source de tant d’inégalités sociales", "Faux"));
    quiz.push(Question::new("Question8 (vrai/faux)- Marx célèbre les Lumières et l’idée de progrès véhiculée par les Lumières", "Vrai"));
    quiz.push(Question::new("Question9 (mots - 1) - Marx constate que le capitalisme reproduit la ____________mots____________ qui se trouve au cœur de l’esclavagisme et au cœur du féodalisme", "Domination"));
    quiz.push(Question::new("Question10 (mots - 1) - Appeler la Révolution française une révolution bourgeoise, c’est, en somme, en faire la ______mots______ : la Révolution française promet beaucoup, dit Marx, mais elle réalise inégalement ses promesses", "Critique"));
    quiz.push(Question::new("Question11 a) (Liste de mots) - Comment Marx appelle-t-il ce que nous appelons aujourd’hui le 1% ?", "la bourgeoisie les riches"));
    quiz.push(Question::new("Question11 b) (Liste de mots) - Comment Marx appelle-t-il ce que nous appelons aujourd’hui le 99%", "le Prolétariat le reste de la population"));
    quiz.push(Question::new("Question12 (Liste de mots) - Quelle est la première personne à avoir dit qu’elle n’est pas marxiste ?", "Karl Marx"));
    quiz.push(Question::new("Question14 (vrai/faux) - Pour Marx, l’être humain est un producteur, exactement comme le l’abeille produit sa ruche ou l’araignée produit sa toile:", "faux"));
    quiz.push(Question::new("Question15 (vrai/faux) - Pour Marx, la spécificité humaine est strictement d’ordre corporel", "vrai"));
    quiz.push(Question::new("Question16 (Liste de mots) - Quelle est la formule marxienne qui résume l’histoire de toutes les sociétés humaines?", "lutte des classes"));
    quiz.push(Question::new("Question17 (Listes de mots) - « Les philosophes n'ont fait qu'interpréter diversement le monde, il s'agit maintenant de le transformer », dit Marx. Pour transformer le monde, Marx formule une exhortation, un encouragement pour inciter les gens. Cette exhortation se trouve à la toute fin du Manifeste du Parti communiste. Quelle est cette exhortation?", "Prolétaires de tous les pays, unissez-vous"));
    quiz.push(Question::new("Question18 (Listes de mots) - Lorsque Marx dit qu’il faut « renverser toutes les conditions sociales dans lesquelles l’homme est un être avili, abandonné, méprisable », il fait la promotion d’une…", "révolution sociale"));
    quiz.push(Question::new("Question19 (Liste de mots) - Le « secret » du capitalisme, explique Marx, c’est, simplement, le travail…", "travail non payée"));
    quiz.push(Question::new("Question20 (Mots) - •Pour Marx, le meilleur moyen d’apaiser les masses populaires, c’est la _____mots_____ , qu’il conçoit exclusivement comme un analgésique. Ainsi analgésiées, les masses populaires ne pourront penser à changer leurs conditions de vie.", "religion"));
    quiz.push(Question::new("Question22 (vrai/faux) - Pour Marx, le travail est nécessairement un mal, un peu comme dans la Genèse (3:19), le premier livre de la Bible, où le travail est présenté comme une peine, une sanction, une punition (« tu gagneras ton pain à la sueur de ton front »)", "faux"));
    quiz.push(Question::new("Question23 (Mots) - Une réforme et une révolution ont en commun de changer les choses (de les réformer, précisément, d’opérer un changement, précisément,  révolutionnaire). La différence, c’est qu’une réforme change les choses avec l’intention de ______mots______ le système en place, alors qu’une révolution change les choses avec l’intention d’______mots______ le système, de le ______mots______ par un nouveau système.", "Conserver, d'abolir, remplacer"));

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..quiz.len());
    let random_question = quiz[random_index];

    Json(random_question)
}
