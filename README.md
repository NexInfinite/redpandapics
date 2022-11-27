# [DEPRECIATED] Red panda pics
Was an old api to get a random image of a redpanda (as they were my favourite animal at one point), since has been deleted and the domain redpanda.pics has been taken by someone else; this is not me. Was a fun mini-project. Please note that **I have no licesnses to any of the images and therefore they have all been deleted from the cdn, to add images please add them to the cdn folder or respective folders in rust**

# Rust
The live version of redpanda.pics running of my server.

### Endpoints
- GET / => Get a random red panda from the cdn.
- GET /boop => Get a random red panda boop!
- GET /tired => Get a yawning/sleeping red panda.
- GET /fact => Get a random fact about redpandas.
- GET /tall => Get a random image of a red panda standing up.
- GET /random => Get a random red panda from the cdn, but it comes as a static image.
- GET /endpoints => Get a list of all endpoints with descriptions and methods.

### Todo
- [ ] GET /random?type=[boop/tired/fact/tall] => To return a static image.
- [ ] GET/POST /upload?token=TOKEN => Upload an image with an authorized token.
- [ ] GET /stats => Get stats about how many images are in the cdn and other things like that.
- [ ] More categories for red pandas.

# Python
> NOTE: No longer in use on live version

See random red pandas

### Endpoints
- GET / => Get a random red panda from the cdn.
- GET /upload?token=TOKEN => Upload a photo
- GET /random => Get a random red panda from the cdn, but it comes as a static image.
