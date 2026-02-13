package db

import (
	"context"
	"net"
	"net/url"
	"strings"
	"time"
)

type Client struct {
	address string
}

func New(databaseURL string) (*Client, error) {
	u, err := url.Parse(databaseURL)
	if err != nil {
		return nil, err
	}
	host := u.Host
	if !strings.Contains(host, ":") {
		host += ":5432"
	}
	return &Client{address: host}, nil
}

func (c *Client) PingContext(ctx context.Context) error {
	d := net.Dialer{Timeout: 2 * time.Second}
	conn, err := d.DialContext(ctx, "tcp", c.address)
	if err != nil {
		return err
	}
	_ = conn.Close()
	return nil
}

func (c *Client) Close() error { return nil }
