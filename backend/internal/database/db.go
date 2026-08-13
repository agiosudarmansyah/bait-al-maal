// Package database is used to initialize a connection to SQLite
package database

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"os"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

type Database struct {
	db *sql.DB
}

func setDriver(key string) (string, string) {
	return "sqlite3", os.Getenv(key)
}

func (pool *Database) Conn() error {
	db, err := sql.Open(setDriver("DATABASE_DSN"))
	if err != nil {
		return err
	}
	db.SetConnMaxLifetime(0)
	db.SetMaxIdleConns(1)
	db.SetMaxOpenConns(1)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := db.PingContext(ctx); err != nil {
		return nil, fmt.Errorf("database connectivity check failed: %w", err)
	}

	pool.db = db

	return nil
}

func (pool *Database) Test() error {
	db, err := sql.Open(setDriver("DATABASE_DSN_TEST"))
	if err != nil {
		log.Fatal()
	}
	db.SetConnMaxLifetime(0)
	db.SetMaxIdleConns(1)
	db.SetMaxOpenConns(1)

	return nil
}
