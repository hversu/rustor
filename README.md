


add the following options to the Target field of the shortcut (replace server and port)

`--proxy-server="socks5://<server>:<port>"`
`--host-resolver-rules="MAP * ~NOTFOUND , EXCLUDE <server>"`

It should now look something like:

`"C:\Program Files\Google\Chrome\Application\chrome.exe" --proxy-server="socks5://<server>:<port>"  --host-resolver-rules="MAP * ~NOTFOUND , EXCLUDE <server>"`
