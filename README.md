# esv-cli

A command line interface for the ESV API.

## Prerequisites

You must have an API token which should either be stored in a `config.toml`, e.g.:
```toml
esv_api_key = <YOUR API KEY>
```
or as an environment variable, e.g.:
```bash
export ESV_API_KEY=<YOUR API KEY>
```

This could also be stored in a `.env` file in the root directory of this repository, e.g.:
```.env
ESV_API_KEY=<YOUR API KEY>
```

For more information on creating an API token, see the API docs linked below.

## Usage

### Building
```bash 
cargo build --release
export PATH="<PATH/TO/REPO>/esv-cli/target/release:$PATH"
```

### Example
```
$ esv-cli "Romans 8:31-37"
Romans 8:31–37

God’s Everlasting Love

  [31] What then shall we say to these things? If God is for us, who can be(1) against us? [32] He who did not spare his own Son but gave him up for us all, how will he not also with him graciously give us all things? [33] Who shall bring any charge against God’s elect? It is God who justifies. [34] Who is to condemn? Christ Jesus is the one who died—more than that, who was raised—who is at the right hand of God, who indeed is interceding for us.(2) [35] Who shall separate us from the love of Christ? Shall tribulation, or distress, or persecution, or famine, or nakedness, or danger, or sword? [36] As it is written,

    “For your sake we are being killed all the day long;
        we are regarded as sheep to be slaughtered.”
    
    
      [37] No, in all these things we are more than conquerors through him who loved us.

Footnotes

(1) 8:31 Or *who is*

(2) 8:34 Or *Is it Christ Jesus who died . . . for us?*
 (ESV)
```

## References
1. https://api.esv.org
2. https://api.esv.org/docs/

## Copyright

Scripture quotations are from the ESV® Bible (The Holy Bible, English Standard Version®), © 2001 by Crossway, a publishing ministry of Good News Publishers. Used by permission. All rights reserved. The ESV text may not be quoted in any publication made available to the public by a Creative Commons license. The ESV may not be translated into any other language.

Users may not copy or download more than 500 verses of the ESV Bible or more than one half of any book of the ESV Bible.