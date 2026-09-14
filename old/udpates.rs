
/*
destructuring
String functions
trait
trait bounds
struct
name
impl
constructor
self
&self
enums
enum types
match
result

option
short hand operator for result/option


concurrency and parallelism
axum server
worker threads
synchrnous
async and await code
tokio runtime / nodejs

os thread handling
mutex
arc
rw lock


*/

// FreeArticle - title & Description,author, type - "free".to_string()
// PaidArticle - title & Description, author,type - "paid".to_string()

// find_author() works both for free and paid only for paid article

// Summary is a feature works only for paid


#[derive(Debug)]
struct Article {
    title:String,
    desc:String,
    author:String,
    article_type:String,
}



trait ArticleType{
    fn find_article_type(&self);
}

impl ArticleType for Article {
    fn find_article_type(&self){
        println!("article author is {}",self.article_type);
    }
}


impl Article{
    fn summarize(&self){
    println!("article summ is {:?}",self.article_type);
}

}
fn summarize<T:ArticleType>(article:T){
    article.summarize();
}

fn main(){
    let fa = Article{
        title:"Global Warming".to_string(),
        desc:"article on gw".to_string(),
        author:"Steve".to_string(),
        article_type:"free".to_string(),
    };
       let pa = Article{
        title:"Fast Fashion Industry".to_string(),
        desc:"article on FFI".to_string(),
        author:"Amy".to_string(),
        article_type:"Paid".to_string(),
    };
    // fa.find_author();
   fa.summarize();
}

// divide - /0 throw an error, cant divide or esle return integer
