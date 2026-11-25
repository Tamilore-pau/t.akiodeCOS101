--
-- PostgreSQL database dump
--

\restrict AUICvBUrLjzCQn1k0zD7DsIc8RD8Z5dkJ5rbxGRh3cv1MNyLHcYJMO1dh40E8MO

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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    sid integer NOT NULL,
    sname text NOT NULL,
    sage integer NOT NULL,
    semail character varying(50) NOT NULL,
    smobile character varying(11)
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (sid, sname, sage, semail, smobile) FROM stdin;
\.


--
-- Name: staff staff_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT staff_pkey PRIMARY KEY (sid);


--
-- PostgreSQL database dump complete
--

\unrestrict AUICvBUrLjzCQn1k0zD7DsIc8RD8Z5dkJ5rbxGRh3cv1MNyLHcYJMO1dh40E8MO

