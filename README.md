## rustor - proxy tor server

# instructions

on your server:

1. install tor and rust
3. manage a firewall to restrict access
4. clone this repo, cd into directory and `cargo run`
5. on the client computer, make a new Chrome shortcut
6. right click: properties on the shortcut
7. add the following options to the Target field of the shortcut (replace server and port)

`--proxy-server="socks5://<server>:<port>"`
`--host-resolver-rules="MAP * ~NOTFOUND , EXCLUDE <server>"`

It should now look something like:

`"C:\Program Files\Google\Chrome\Application\chrome.exe" --proxy-server="socks5://<server>:<port>"  --host-resolver-rules="MAP * ~NOTFOUND , EXCLUDE <server>"`


enjoy browsing the dark web through your proxy!
