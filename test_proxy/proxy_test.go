package main

import (
	"bufio"
	"fmt"
	"net"
	"testing"
)

var (
	ports    = []string{"7001", "7200", "7300", "7400"}
	messages = []string{"PING\n", "PING\n", "SET cookie secret\n", "GET cookie\n"}
)

// Test all port connections
func TestTraffic(t *testing.T) {
	var connections []net.Conn

	for _, port := range ports {
		socket_addr := fmt.Sprintf("localhost:%s", port)
		conn, err := net.Dial("tcp", socket_addr)
		if err != nil {
			fmt.Printf("could not connect to %s\n", socket_addr)
			continue
		}
		connections = append(connections, conn)
	}

	for idx, conn := range connections {
		response := testConn(conn, messages[idx])
		t.Logf("Response: %v", response)
	}
}

// Test retry (six-thousand app)
func TestRetry(t *testing.T) {

	var connections []net.Conn
	for i := 0; i < 4; i++ {
		socket_addr := fmt.Sprintf("localhost:%s", ports[i])
		conn, err := net.Dial("tcp", socket_addr)
		if err != nil {
			fmt.Printf("could not connect to %s\n", socket_addr)
			continue
		}
		connections = append(connections, conn)
	}

	for idx, conn := range connections {
		if response := testConn(conn, messages[idx]); response == "" {
			t.Errorf("Could not find target server")
		} else {
			t.Logf("Target server found!\n")
		}
	}

}

// Helper function to simulate customer traffic
func testConn(conn net.Conn, msg string) string {
	defer conn.Close()
	conn.Write([]byte(msg))
	response, err := bufio.NewReader(conn).ReadString('\n')

	if err != nil {
		return ""
	}

	return response
}
