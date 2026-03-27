**running locally**

cargo run

or with docker

docker-compose up

server runs on http://0.0.0.0:7700

**post /indexes**

create index

curl -X POST http://localhost:7700/indexes \
  -H "Content-Type: application/json" \
  -d '{"name":"products"}'

**get /indexes**

list indexes

curl http://localhost:7700/indexes

**delete /indexes/:name**

delete index

curl -X DELETE http://localhost:7700/indexes/products

**post /indexes/:name/documents**

add documents (single or batch)

curl -X POST http://localhost:7700/indexes/products/documents \
  -H "Content-Type: application/json" \
  -d '{"id":"1","title":"red shoes","price":50}'

batch

curl -X POST http://localhost:7700/indexes/products/documents \
  -H "Content-Type: application/json" \
  -d '[{"id":"1","title":"red shoes"},{"id":"2","title":"blue shirt"}]'

**get /indexes/:name/search**

search documents

curl "http://localhost:7700/indexes/products/search?q=red+shoes"

fuzzy matching

curl "http://localhost:7700/indexes/products/search?q=red+shoes&fuzzy=true"

pagination

curl "http://localhost:7700/indexes/products/search?q=red&limit=10&offset=0"
