# riichi-in-rust
v0.0.43

Rust + WASM. I wanted to learn these exciting technologies. What project to pick though?
Well, since I like to play Riichi, why not something like an analyzer for
hands and tables in mahjong!

My ultimate goal is for this to be a WASM library, that can be included into
any project you like and can just work - send in data in JSON and get back JSON data
again. 

What I really like about this is that after implementing a frontend, the whole
thing will work offline, in the client, no serverside stuff needed at all.
Only the frontend will be in Javascript+HTML, so no logic in JS, all in Rust, hooray!

Lots of inspiration from other mahjong tools, like https://euophrys.itch.io/mahjong-efficiency-trainer, http://tenhou.net/2/, http://kobalab.net/majiang/dapai.html etc.

## Roadmap
- South 4 Simulator
    - A game where you try to win in the last round
- Hand analysis
    - Shanten for 13 tiles [DONE]
    - Shanten for 14 tiles with complete hand indication [DONE]
    - Ukeire, tile acceptance
    - Potential discards and their value    
    - Hand value for complete hand (14 tiles)
    - Hand value for tenpai hand with possible outcomes
    - All of the above also for hands with calls
- Table analysis
    - Safe tiles
    - Wait probability percentages
- Replay analysis
    - Tenhou replay parsing
    - Majsoul replay parsing
    - Discard rating
