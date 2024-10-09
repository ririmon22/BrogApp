/// クエリモジュール
/// 
/// このファイルは、Dieselを使用してユーザー、投稿、コメントのCRUD操作を行うための関数を提供します。
/// データベースはMySQLを使用し、Dieselのクエリビルダを使ってデータベースとのやり取りを行います。
/// ユーザー、投稿、コメントの作成(create)および削除(delete)操作を行います。
/// 
/// 使用するフレームワークとライブラリ：
/// - crate/models:    自作ライブラリ モデル定義用ファイル
/// - crate/db/schema: 自作ライブラリ Dieselのスキーマ定義用ファイル(自動生成)
/// - diesel/prelude : クエリ構築な必要なメソッド、構造体をインポート
/// - diesel/MyssqlConnection   : MySQL接続用のオブジェクト、preludeをインポートしていれば必要ないが明示的にするため
/// - diesel/result/QueryResult : クエリ結果を扱うための型、preludeをインポートしていれば必要ないが明示的にするため
///

use crate::models; 
use crate::db::schema; 
use diesel::prelude::*; 
use diesel::MysqlConnection; 
use diesel::result::QueryResult;

/// 新しいユーザーを作成する関数
///
/// 指定された `name`、`email`、`password_hash` を持つ新しいユーザーをデータベースに挿入します。
/// 成功した場合、挿入したユーザーを返します。
///
/// # 引数
/// - `conn`: データベース接続用のMysqlConnectionオブジェクト
/// - `name`: ユーザーの名前
/// - `email`: ユーザーのメールアドレス
/// - `password_hash`: ユーザーのパスワードハッシュ
///
/// # 戻り値
/// - 作成された `User` オブジェクト
pub fn create_user(conn: &mut MysqlConnection, name: &str, email: &str, password_hash: &str) -> QueryResult<models::User> {
    let new_user = models::NewUser {
        name: name.to_string(),
        email: email.to_string(),
        password_hash: password_hash.to_string(),
    };

    // 新しいユーザーをテーブルに挿入
    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)?;

    // 最後に挿入されたユーザーを取得
    schema::users::table.order(schema::users::user_id.desc()).first(conn)
}

/// ユーザーを削除する関数
///
/// 指定された `user_id` を持つユーザーをデータベースから削除します。
///
/// # 引数
/// - `conn`: データベース接続用のMysqlConnectionオブジェクト
/// - `user_id`: 削除するユーザーのID
///
/// # 戻り値
/// - 削除された行数
pub fn delete_user(conn: &mut MysqlConnection, user_id: i32) -> QueryResult<usize> {
    let affected_rows = diesel::delete(schema::users::table.find(user_id)).execute(conn)?;
    if affected_rows == 0 {
        println!("No user with id {} found", user_id);
    }
    Ok(affected_rows)
}

/// 新しい投稿を作成する関数
///
/// ユーザーが作成する投稿をデータベースに挿入します。投稿にはタイトル、本文、公開ステータスが含まれます。
///
/// # 引数
/// - `conn`: データベース接続用のMysqlConnectionオブジェクト
/// - `title`: 投稿のタイトル
/// - `body`: 投稿の本文
/// - `is_published`: 投稿が公開されているかどうか
/// - `user_id`: 投稿を作成したユーザーのID
///
/// # 戻り値
/// - 作成された `Post` オブジェクト
pub fn create_post(conn: &mut MysqlConnection, title: &str, body: &str, is_published: bool, user_id: i32) -> QueryResult<models::Post> {
    let new_post = models::NewPost {
        title: title.to_string(),
        post_body: body.to_string(),
        published: is_published,
        user_id,
    };

    // 新しい投稿をテーブルに挿入
    diesel::insert_into(schema::posts::table)
        .values(&new_post)
        .execute(conn)?;

    // 最後に挿入された投稿を取得
    schema::posts::table.order(schema::posts::post_id.desc()).first(conn)
}

/// 投稿を削除する関数
///
/// 指定された `post_id` を持つ投稿をデータベースから削除します。
///
/// # 引数
/// - `conn`: データベース接続用のMysqlConnectionオブジェクト
/// - `post_id`: 削除する投稿のID
///
/// # 戻り値
/// - 削除された行数
pub fn delete_post(conn: &mut MysqlConnection, post_id: i32) -> QueryResult<usize> {
    let affected_rows = diesel::delete(schema::posts::table.find(post_id)).execute(conn)?;
    if affected_rows == 0 {
        println!("No post with id {} found", post_id);
    }
    Ok(affected_rows)
}

/// 新しいコメントを作成する関数
///
/// 指定された `user_id`、`post_id`、`body` を持つコメントをデータベースに挿入します。
///
/// # 引数
/// - `conn`: データベース接続用のMysqlConnectionオブジェクト
/// - `user_id`: コメントを作成したユーザーのID
/// - `post_id`: コメントが関連する投稿のID
/// - `body`: コメントの本文
///
/// # 戻り値
/// - 作成された `Comment` オブジェクト
pub fn create_comment(conn: &mut MysqlConnection, user_id: i32, post_id: i32, body: &str) -> QueryResult<models::Comment> {
    let new_comment = models::NewComment {
        user_id,
        post_id,
        comment_body: body.to_string(), 
    };

    // 新しいコメントをテーブルに挿入
    diesel::insert_into(schema::comments::table)
        .values(&new_comment)
        .execute(conn)?;

    // 最後に挿入されたコメントを取得
    schema::comments::table.order(schema::comments::comment_id.desc()).first(conn)
}

/// コメントを削除する関数
///
/// 指定された `id` を持つコメントをデータベースから削除します。
///
/// # 引数
/// - `conn`: データベース接続用のMysqlConnectionオブジェクト
/// - `id`: 削除するコメントのID
///
/// # 戻り値
/// - 削除された行数
pub fn delete_comment(conn: &mut MysqlConnection, id: i32) -> QueryResult<usize> {
    let affected_rows = diesel::delete(schema::comments::table.find(id)).execute(conn)?;
    if affected_rows == 0 {
        println!("No comment with id {} found", id);
    }
    Ok(affected_rows)
}
