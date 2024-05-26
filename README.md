## rustor

# instructions

on your server:

1. install tor and rust

2. manage a firewall to restrict access

3. clone this repo, cd into directory and `cargo run`

4. on the client computer, make a new Chrome shortcut
5.  add the following options to the Target field of the shortcut (replace server and port)

`--proxy-server="socks5://<server>:<port>"`
`--host-resolver-rules="MAP * ~NOTFOUND , EXCLUDE <server>"`

It should now look something like:

`"C:\Program Files\Google\Chrome\Application\chrome.exe" --proxy-server="socks5://<server>:<port>"  --host-resolver-rules="MAP * ~NOTFOUND , EXCLUDE <server>"`


enjoy browsing the dark web through your proxy!
