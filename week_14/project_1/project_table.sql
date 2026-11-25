--
-- PostgreSQL database dump
--

\restrict 0SR5JJj9wonA1nmW6TM1ChFtsXdPJmenUAbBxKosqYpXg0RSzMrEyh8HQz3GUYG

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pid integer NOT NULL,
    pname text NOT NULL,
    pduration date NOT NULL,
    eid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pid, pname, pduration, eid) FROM stdin;
\.


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pid);


--
-- PostgreSQL database dump complete
--

\unrestrict 0SR5JJj9wonA1nmW6TM1ChFtsXdPJmenUAbBxKosqYpXg0RSzMrEyh8HQz3GUYG

