/// モデルモジュール
/// 
/// アプリケーションで使用するデータモデルを定義します。
/// モデルは構造体で定義し、データベースとやり取りするためにDieselのトレイトを実装します。
/// さらに、必要に応じてゲッターメソッドを提供し、フィールドにアクセスできるようにします。
///
/// 使用するフレームワークとライブラリ：
/// - diesel: データベースと接続し、ORMとして動作するため。
/// - serde: モデルの構造体をJSON形式にシリアライズ・デシリアライズする場合に使用可能です。　拡張用
///
/// それぞれの構造体はschema.rsのテーブルと紐付けされDB操作に使用する。

use diesel::prelude::*;
use crate::db::schema::*;

/// ユーザーモデル
/// ユーザーに関する情報を保持
#[derive(Identifiable, Queryable)]
#[diesel(table_name = users)]  
#[primary_key(user_id)]
pub struct User {
    user_id: i32,
    name: String,
    email: String,
    password_hash: String,
}

/// 外部からUser構造体へのアクセスを制限するためにゲッターメソッドを定義
impl User {

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

/// 投稿モデル
/// ユーザーが作成する投稿を保持する
/// 投稿はユーザーに関連付けられ、タイトル、本文、公開ステータスなどの情報を持つ。

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
#[diesel(table_name = posts)]
#[primary_key(post_id)]
pub struct Post {
    post_id: i32,
    title: String,
    post_body: String,
    published: bool,
    user_id: i32,
}

/// 外部からPost構造体へのアクセスを制限するためにゲッターメソッドを定義
impl Post {
    pub fn post_id(&self) -> i32 {
        self.post_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn post_body(&self) -> &str {
        &self.post_body
    }

    pub fn published(&self) -> bool {
        self.published
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

/// コメントモデル
/// 作成した投稿へのコメントを保持する
/// コメントはユーザーと投稿に関連付けられ、ユーザーID,投稿ID,コメント本文の情報を持つ。

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[diesel(table_name = comments)]
#[primary_key(comment_id)]
pub struct Comment {
    comment_id: i32,
    post_id: i32,
    user_id: i32,
    comment_body: String,
}

impl Comment {
    pub fn comment_id(&self) -> i32 {
        self.comment_id
    }

    pub fn post_id(&self) -> i32 {
        self.post_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn comment_body(&self) -> &str {
        &self.comment_body
    }
}

/// 以下の構造体はそれぞれのモデルにデータを挿入する際に使用する。
/// 主キーであるそれぞれのIDはデータベース側でオートインクリメントを行う仕様としているためデータ挿入には使用しない。

#[derive(Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub post_body: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub user_id: i32,
    pub post_id: i32,
    pub comment_body: String,
}