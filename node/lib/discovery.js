
import ffi from "ffi"
import ref from "ref"
import Struct from "ref-struct"
import RefArray from "ref-array"

let StringArray = RefArray("string")
let StringArrayPtr = ref.refType(StringArray)

let ServiceDescriptor = Struct({
  id: 'string',
  identifier: 'string',
  tags: StringArray,
  codecs: StringArray,
  connection_urls: StringArray,
  tags_length: 'size_t',
  codecs_length: 'size_t',
  connection_urls_length: 'size_t'
});

export default class NativeDiscovery {

  constructor(libraryName, prefix) {

    var def = {}

    def[func(prefix, "create")] = ["pointer", ['string']]
    def[func(prefix, "get_service_with_tags")] = [ServiceDescriptor, ["pointer"]]
    def[func(prefix, "advertise_local_service")] = ["void", ["pointer", ServiceDescriptor]]
    def[func(prefix, "on_ready")] = ["void", ["pointer", "pointer"]]

    this.lib = ffi.Library(libraryName, def)
    this.disco = this.lib.create("WOOT!!! YO DUD")
  }

  connect(onReady, addFoundServices, serviceList) {
    var callback = ffi.Callback("void", [], onReady);
    this.lib.on_ready(this.disco, callback)
    process.on('exit', function() {
      callback
    });
  }

  requestServiceList(onServiceList) {

  }

  shutdown() {
    this.lib.shutdown()
  }

  listenToServices() {

  }

  announce(instanceDescriptor) {
    let tags = new StringArray(instanceDescriptor.tags)
    let codecs = new StringArray(instanceDescriptor.codecs)
    let connection_urls = new StringArray(instanceDescriptor.connection_urls)

    var service = new ServiceDescriptor();
    service.identifier = instanceDescriptor.identifier
    service.id = instanceDescriptor.id
    service.tags = tags
    service.codecs = codecs
    service.connection_urls = connection_urls
    service.tags_length = tags.length
    service.codecs_length = codecs.length
    service.connection_urls_length = connection_urls.length

    this.lib.advertise_local_service(this.disco, service)
  }
}

function func(prefix, name) {
  return prefix + "_" + name
}
