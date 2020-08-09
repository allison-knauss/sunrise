# Sunrise Routing

Routing in `sunrise` is not a single service, but a set of patterns that allow routing. 

## Custom DNS

At its core, routing is nothing more than custom DNS. If you need a service, you simply request the known endpoint for it, and if something is listening there, it will respond. So for `aaa` requests, you may hit the `tcp://aaa/` endpoint.

As a general pattern, all services listen on `$PROTOCOL://$SERVICE:$ENV_PORT/`

## Port environments

Each environment has a port associated with it. ALL services in that environment use that port.

## Rationale

There was initially to be a rich router service here. But that would create a single point of failure for the entire system, and lots of plumbing work, so I threw it out. The custom DNS approach eliminates single points of failure and star-pattern problems.

## Other Advantages

### Easy docker-compose Integration

One advantage of this approach is that docker-compose can be trivially configured to run our services in the exact kind of custom private network DNS we need.

### Requisite Security

Another advantage is that there is no clear way presented to directly connect all the components to the public internet. This improves security by adding friction to not having some sort of proxy in the middle.
