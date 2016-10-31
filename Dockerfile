# iwdb_mongo image
#   zpallin
#   2016
#
# right now there is no difference from the typical mongo container, but I will update this
# to include base scaffolding at some point

FROM fnichol/rust:1.12.0
ADD ./ /src/iwdb
WORKDIR /src/iwdb
EXPOSE 3000
RUN cargo build
CMD cargo run --example basic
