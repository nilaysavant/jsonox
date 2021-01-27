/// Pick Colors from: https://stackoverflow.com/questions/5947742/how-to-change-the-output-color-of-echo-in-linux
pub fn get_tutorial_text() -> String {
    format!(
        "
GUIDE (normal mode):
    {green}POST or PUT data to an arbitrary endpoint. You can the GET the posted data on the same endpoint:{nc}
        {purple}POST{nc}/{purple}PUT{nc}: {orange}{{ \"cute\" : true }}{nc} => {cyan}/pets/cat{nc}
        {purple}GET{nc}: {cyan}/pets/cat{nc} => {orange}{{ \"cute\" : true }}{nc}

    {green}Also Delete the stored data:{nc}
        {purple}DELETE{nc}: {cyan}/pets/cat{nc} => {orange}{{ \"cute\" : true }}{nc}

    {green}The above will setup files under the{nc} {cyan}./jsonox_data{nc}{green}:{nc}
        {cyan}- pets/
            - cat/
                - index.json{nc}
    {green}Root endpoint `/` will display all active endpoints:{nc}
        {purple}GET{nc}: {cyan}/{nc} => {orange}{{ \"active_paths\": [ \"pets/cat\" ] }}{nc}

    {green}You can setup your own files in the above structure:{nc}
        {cyan}- pets/
            - dog/
                - index.json
            - cat/
                - index.json
            - index.json
        - toys/
            - doll/
                - index.json{nc}
    {green}Then:{nc}
        {purple}GET{nc}: {cyan}/{nc} => {orange}{{ \"active_paths\": [ \"pets\", \"pets/cat\", \"pets/dog\", \"toys/doll\" ] }}{nc}
    
    {green}You can then do{nc} {purple}GET/POST/PUT/DELETE{nc} {green}on the endpoint paths above.{nc}

GUIDE (Read Only mode):
    {green}Start by creating files in {cyan}./jsonox_data{nc}:{nc}
        {cyan}- pets/
            - dog/
                - get.json
                - post.json
            - cat/
                - get.json
            - get.json
            - delete.json
        - toys/
            - doll/
                - get.json
                - post.json
                - put.json
                - delete.json{nc}
    {green}In Read Only mode we create files like get.json, 
    post.json, put.json, and delete.json, instead of index.json{nc}
    {green}Thus as per above structure:{nc}
        {cyan}/pets{nc} will have {purple}GET{nc} and {purple}DELETE{nc} only.
        {cyan}/pets/cat{nc} will have {purple}GET{nc} only.
        {cyan}/pets/dog{nc} will have {purple}GET{nc} and {purple}POST{nc} only.
        {cyan}/toys/doll{nc} will have {purple}GET{nc}, {purple}POST{nc}, {purple}PUT{nc} and {purple}DELETE{nc}.
    
    {green}For viewing all active paths (is same as normal mode):{nc}
        {purple}GET{nc}: {cyan}/{nc} => {orange}{{ \"active_paths\": [ \"pets\", \"pets/cat\", \"pets/dog\", \"toys/doll\" ] }}{nc}

",
        // red = "\x1b[0;31m",
        // lred = "\x1b[1;31m",
        green = "\x1b[0;32m",
        orange = "\x1b[0;33m",
        // yellow = "\x1b[1;33m",
        // blue = "\x1b[0;34m",
        // lblue = "\x1b[1;34m",
        purple = "\x1b[0;35m",
        cyan = "\x1b[0;36m",
        // lgrey = "\x1b[0;37m",
        nc = "\x1b[0m",
    )
}
