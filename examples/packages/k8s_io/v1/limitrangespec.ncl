# Module: k8s.io.v1.limitrangespec

let limitrangeitem = import "./limitrangeitem.ncl" in

{
  # LimitRangeSpec defines a min/max usage limit for resources that match on kind.
  LimitRangeSpec = {
    limits | Array limitrangeitem.LimitRangeItem | doc "Limits is the list of LimitRangeItem objects that are enforced.",
  },
}
