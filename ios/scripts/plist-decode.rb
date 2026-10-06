# Native XML plist decoder, including NSDate values unsupported by JSON conversion.
require 'rexml/document'
def plist_value(node)
  case node.name
  when 'dict'
    node.elements.to_a.each_slice(2).to_h { |key, value| [key.text, plist_value(value)] }
  when 'array'
    node.elements.map { |value| plist_value(value) }
  when 'true', 'false'
    node.name == 'true'
  else
    node.text.to_s
  end
end
def plist(data)
  plist_value(REXML::Document.new(command('plutil', '-convert', 'xml1', '-o', '-', '-', input: data)).root.elements[1])
end