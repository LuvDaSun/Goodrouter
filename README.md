# Goodrouter, TypeScript edition

A good router should:

- [x] work in a server or a client (or any other) environment
- [x] be able to construct routes based on their name
- [x] should have a simple API!
- [x] not do the actual navigation!
- [x] be framework agnostic
- [x] be very minimal and simple!

Check out our [website](https://www.goodrouter.org). And feel free to join our [Discord server](https://discord.gg/BJ8v7xTq8d)!

## Releasing

Releasing a new version should be done manually. Depending on the package you are releasing there is a slightly different process.

### net/Goodrouter

Make sure you have dotnet sdk 6.0 installed. Also install nuget.

Bump the versions in `packages/net/Goodrouter/Goodrouter.csproj`.

Then create a package via

```sh
dotnet pack
```

Then commit your changes and push.

And publish the package via

```sh
dotnet nuget push packages/net/Goodrouter/bin/Release/*.nupkg
```

### rs/goodrouter

You need rust and cargo installed. Also install cargo-edit.

Then update the package version via (of course you can also bump minor of major)

```sh
cargo set-version --bump patch --package goodrouter
```

Then commit and push your changes to git.

Then publish the package to the registry

```sh
cargo publish --package goodrouter
```

### ts/goodrouter

Bump the package version via

```sh
npm --workspace goodrouter version patch
```

Then commit and push to git.

The publish the package via

```sh
npm --workspace goodrouter publish
```

### ts/www

You need to have the aws cli installed. And you need to be authenticated and authorized! Then install everything via `npm install`, then build the project via

```sh
npm --workspace www run build
```

Then publish the website via

```sh
aws s3 sync packages/ts/www/out s3://www.goodrouter.org --delete
```
