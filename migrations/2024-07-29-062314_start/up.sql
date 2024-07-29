-- Your SQL goes here
CREATE TABLE "communities"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"slug" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"deleted" BOOL NOT NULL,
	"suspended" BOOL NOT NULL,
	"restricted" BOOL NOT NULL,
	"created_on" TIMESTAMP NOT NULL,
	"deleted_on" TIMESTAMP NOT NULL,
	"suspended_on" TIMESTAMP NOT NULL,
	"restricted_on" TIMESTAMP NOT NULL
);

CREATE TABLE "posts"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"body" TEXT NOT NULL,
	"post_type" SMALLINT NOT NULL,
	"score" INTEGER NOT NULL,
	"deleted" BOOL NOT NULL,
	"removed" BOOL NOT NULL,
	"edited" BOOL NOT NULL,
	"locked" BOOL NOT NULL,
	"archived" BOOL NOT NULL,
	"created_on" TIMESTAMP NOT NULL,
	"edited_on" TIMESTAMP NOT NULL,
	"deleted_on" TIMESTAMP NOT NULL,
	"removed_on" TIMESTAMP NOT NULL,
	"locked_on" TIMESTAMP NOT NULL,
	"archived_on" TIMESTAMP NOT NULL
);

CREATE TABLE "users"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"email" TEXT NOT NULL,
	"username" TEXT NOT NULL,
	"display_name" TEXT NOT NULL,
	"bio" TEXT NOT NULL,
	"pw_hash" TEXT NOT NULL,
	"pronouns" TEXT NOT NULL,
	"deleted" BOOL NOT NULL,
	"suspended" BOOL NOT NULL,
	"locked" BOOL NOT NULL,
	"created_on" TIMESTAMP NOT NULL,
	"deleted_on" TIMESTAMP NOT NULL,
	"suspended_on" TIMESTAMP NOT NULL,
	"locked_on" TIMESTAMP NOT NULL
);

