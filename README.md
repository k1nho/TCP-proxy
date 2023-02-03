# TCP Proxy

A raw configurable TCP proxy which listens to the ports given in a json config file and routes connections to an app's target.

## Using the Proxy

- Configure the proxy with the config.json file.
```json
{
  "Apps": [
    {
      "Name": "Vite App",
      "Ports": [
        7001,
        7200,
      ],
      "Targets": [
        "localhost:6379",
        "server:7003"
      ]
    },
    {
      "Name": "Next App",
      "Ports": [
        6200,
        6300,
        6400
      ],
      "Targets": [
        "server:6001",
        "server:6002",
        "localhost:8080"
      ]
    }
  ]
}
```

- To build and run the project, the following command can be used:

```console
// This command will create the binary by executing cargo build and run the binary  
root@root-MBP tcp_proxy % make all

// Alternatively, you can make use of these two commands to achieve the same thing 
root@root-MBP tcp_proxy % make build
root@root-MBP tcp_proxy % make run
```

- After running the commands, a directory called **/prod** will be created with the binary executable from cargo build and the config.json file. Next, the tcp\_proxy binary will be run
and it will listen to the ports given by the config file. Below is an example output after running **make run** command. 

```console
root@root-MBP tcp_proxy % make run
prod/tcp_proxy
```

- You can use the echo command along with the pipe operator to send data to the netcat utility as following: `echo "hello world" | nc localhost 7001 -w 1`. 
Below is an example output. 

```console
root@root-MBP tcp_proxy % echo "hello world" | nc localhost 7001 -w 1
hello world
```

```console
// Program output
root@root-MBP tcp_proxy % make run
prod/tcp_proxy
Proxying :7001 to localhost:6397
```

