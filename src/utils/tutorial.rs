pub const TUTORIAL_TEXT: &str = "
POST data to an arbitrary endpoint. You can the GET the posted data on the same endpoint:
- POST: { \"cute\" : true } => `/pets/cat`
- GET: `/pets/cat` => { \"cute\" : true }

Also Delete the stored data:
- DELETE: `/pets/cat` => { \"cute\" : true }

The above will setup files under the `./jsonox_data`:
- pets/
    - cat/
        - index.json

Root endpoint `/` will display all active endpoints:
- GET: `/` => { \"active_paths\": [ \"pets/cat\" ] }

You can setup your own files in the above structure:
- pets/
    - dog/
        - index.json
    - cat/
        - index.json
    - index.json
- toys/
    - doll/
        - index.json
Then:
- GET: `/` => { \"active_paths\": [ \"pets\", \"pets/cat\", \"pets/dog\", \"toys/doll\" ] }
You can then do GET/POST/DELETE on the endpoint paths above.
";
