
import NativeDisco from "./lib/discovery"


let disco = new NativeDisco('../target/debug/libmuon_discovery_net')

disco.connect(function() {
 console.log("WOWOWOWOWOWOWOWOWO")
})

disco.announce({
  id: "Mine",
  identifier: "Simples",
  tags: ["hello", "world"],
  connection_urls: ["Awesome coolness"],
  codecs: ["application/json"]
})

setTimeout(function() {
    console.log("HELLO!")
}, 10000)

