DROP TABLE IF EXISTS boards CASCADE;
DROP TABLE IF EXISTS posts CASCADE;
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE boards(
    board_id INT GENERATED ALWAYS AS IDENTITY,
    board_name VARCHAR(255) NOT NULL,
    
    PRIMARY KEY(board_id)
);

CREATE TABLE posts(
    post_id INT GENERATED ALWAYS AS IDENTITY,
    board_id INT NOT NULL,
    author_id INT NOT NULL,
    post_title VARCHAR(255) NOT NULL,
    post_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(post_id),

    CONSTRAINT fk_board
        FOREIGN KEY(board_id) 
            REFERENCES boards(board_id)
);

CREATE TABLE comments(
    comment_id INT GENERATED ALWAYS AS IDENTITY,
    post_id INT NOT NULL,
    replies_to INT,
    author_id INT NOT NULL,
    comment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    comment_text TEXT NOT NULL,

    PRIMARY KEY(comment_id),

    CONSTRAINT fk_post
        FOREIGN KEY(post_id) 
            REFERENCES posts(post_id),

    CONSTRAINT fk_replies_to
        FOREIGN KEY(replies_to)
            REFERENCES comments(comment_id)
);

CREATE TABLE users(
    user_id INT GENERATED ALWAYS AS IDENTITY,
    ip_address VARCHAR(255) NOT NULL
);

INSERT INTO boards (board_name) VALUES ('the first board')