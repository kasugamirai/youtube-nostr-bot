## Setup & Running

1. Change the current working directory to `youtube_db`.
2. Modify `conf/test/conf.template` as per your requirements.
3. Run the following command to apply database migrations:
```shell
cargo install diesel_cli
```
 ```shell
diesel migration run
```
4. Change the working directory back to the project root and build your application by running `build.sh`.
```shell
sh build.sh
```
5. Navigate to the `output` directory:
6. Finally, you can run your application using the provided `youtube_fetch.sh` script.
