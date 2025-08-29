# Module: k8s.io.v1.httpingressrulevalue

let httpingresspath = import "./httpingresspath.ncl" in

{
  # HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
  HTTPIngressRuleValue = {
    paths | Array httpingresspath.HTTPIngressPath | doc "paths is a collection of paths that map requests to backends.",
  },
}
