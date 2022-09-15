
Here is how I see the next steps. No order:

* Assume our DX/guides is setting `PYTHONPATH`. Update README.
* Whatever that path is, leverage it in our shell script tests during setup?
* Make sure all packaging Dockerfiles are using scratch vs aplpine.
* Leverage the devcontainer's latest Python for our work. 
* Avoid more Python coupling to packaging Dockerfiles.
* Test python 2.7 using the `Dockerfile-test` pattern we have in place.
* Create a local Python package with wrapt and loader in /python dir.

Our bin/build should be extended do something like this with local Python.

```shell
pip install . --prefix /opt/crypteia/python/
```

So Michele was doing that pip install and prefix in each Dockerfile-amzn and debian using stuff like this with another build stage.

```dockerfile
ADD ./src/python/crypteia /code
WORKDIR /code
RUN pip install . --prefix /opt/crypteia/python/
# ...
COPY --from=0 /opt/crypteia/python/ /opt/crypteia/python/
COPY ./src/python/usercustomize.py /opt/crypteia/python/usercustomize.py
# ...
```

I'd like to run pip outside in our build script and simply copy into place afterward. So thate prefix may have to change?

This was added to lib.rs but it should not be needed since we are going to push the interface to the client install in the Dockerfile. But maybe we should have this for Lambda Extensions zip?

```rust
if (name == "PYTHONPATH") {
  // Add the path to the crypteia python site that contains
  // additional libraries we need to resolve end vars in CPython
  // runtimes.
  if env_value.is_null() {
    return "/opt/crypteia/python";
  }

  return "/opt/crypteia/python:" + env_value; // FIX THIS?
}
```

Here are some links that may be helpful.

- https://github.com/customink/crypteia/pull/19
- https://github.com/lumigo-io/opentelemetry-python-distro
- https://github.com/lumigo-io/opentelemetry-python-distro/blob/main/setup.cfg
- https://python-packaging-tutorial.readthedocs.io/en/latest/setup_py.html
- https://github.com/tdstark/url_shortener
- https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html
- https://www.dilex.net/data-blog/using-aws-lambda-extensions-with-python

