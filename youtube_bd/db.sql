--
-- PostgreSQL database dump
--

-- Dumped from database version 14.10 (Homebrew)
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: xy
--

-- *not* creating schema, since initdb creates it


ALTER SCHEMA public OWNER TO xy;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: xy
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO xy;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: xy
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO xy;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: xy
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO xy;

--
-- Name: videos; Type: TABLE; Schema: public; Owner: xy
--

CREATE TABLE public.videos (
    id integer NOT NULL,
    author character varying NOT NULL,
    channel character varying NOT NULL,
    title character varying NOT NULL,
    link character varying NOT NULL,
    published boolean DEFAULT false NOT NULL,
    userid integer NOT NULL
);


ALTER TABLE public.videos OWNER TO xy;

--
-- Name: videos_id_seq; Type: SEQUENCE; Schema: public; Owner: xy
--

CREATE SEQUENCE public.videos_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.videos_id_seq OWNER TO xy;

--
-- Name: videos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: xy
--

ALTER SEQUENCE public.videos_id_seq OWNED BY public.videos.id;


--
-- Name: youtube_users; Type: TABLE; Schema: public; Owner: xy
--

CREATE TABLE public.youtube_users (
    id integer NOT NULL,
    username character varying NOT NULL,
    publickey character varying NOT NULL,
    privatekey character varying NOT NULL,
    channel character varying NOT NULL
);


ALTER TABLE public.youtube_users OWNER TO xy;

--
-- Name: youtube_users_id_seq; Type: SEQUENCE; Schema: public; Owner: xy
--

CREATE SEQUENCE public.youtube_users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.youtube_users_id_seq OWNER TO xy;

--
-- Name: youtube_users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: xy
--

ALTER SEQUENCE public.youtube_users_id_seq OWNED BY public.youtube_users.id;


--
-- Name: videos id; Type: DEFAULT; Schema: public; Owner: xy
--

ALTER TABLE ONLY public.videos ALTER COLUMN id SET DEFAULT nextval('public.videos_id_seq'::regclass);


--
-- Name: youtube_users id; Type: DEFAULT; Schema: public; Owner: xy
--

ALTER TABLE ONLY public.youtube_users ALTER COLUMN id SET DEFAULT nextval('public.youtube_users_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: xy
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2024-01-27 17:15:13.84118
20240123053146	2024-01-27 17:15:13.858753
20240124153003	2024-01-27 17:15:13.86449
\.


--
-- Data for Name: videos; Type: TABLE DATA; Schema: public; Owner: xy
--

COPY public.videos (id, author, channel, title, link, published, userid) FROM stdin;
\.


--
-- Data for Name: youtube_users; Type: TABLE DATA; Schema: public; Owner: xy
--

COPY public.youtube_users (id, username, publickey, privatekey, channel) FROM stdin;
\.


--
-- Name: videos_id_seq; Type: SEQUENCE SET; Schema: public; Owner: xy
--

SELECT pg_catalog.setval('public.videos_id_seq', 1, false);


--
-- Name: youtube_users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: xy
--

SELECT pg_catalog.setval('public.youtube_users_id_seq', 1, false);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: xy
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: videos videos_pkey; Type: CONSTRAINT; Schema: public; Owner: xy
--

ALTER TABLE ONLY public.videos
    ADD CONSTRAINT videos_pkey PRIMARY KEY (id);


--
-- Name: youtube_users youtube_users_pkey; Type: CONSTRAINT; Schema: public; Owner: xy
--

ALTER TABLE ONLY public.youtube_users
    ADD CONSTRAINT youtube_users_pkey PRIMARY KEY (id);


--
-- Name: videos videos_userid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: xy
--

ALTER TABLE ONLY public.videos
    ADD CONSTRAINT videos_userid_fkey FOREIGN KEY (userid) REFERENCES public.youtube_users(id);


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: xy
--

REVOKE USAGE ON SCHEMA public FROM PUBLIC;
GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

