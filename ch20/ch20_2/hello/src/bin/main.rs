use hello;

fn main() {
    // ブラウザの１つのタブで /sleep にリクエストした後、別のタブで / にリクエストしたら、２回目のリクエストは待たされる
    // hello::simple_listener();

    // ブラウザの１つのタブで /sleep にリクエストした後、別のタブで / にリクエストしたら、２回目のリクエストは素早く返ってくる
    hello::making_new_threads_without_any_limit();
}