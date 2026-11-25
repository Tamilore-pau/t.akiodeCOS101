--
-- PostgreSQL database dump
--

\restrict 2P4IIb7LBBWhnq8wTNrIarOe4ywCYoUNTvJdVwBZcgsXkNnVVC2NhbviZqyoLEd

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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    cid integer NOT NULL,
    cname text NOT NULL,
    cage integer NOT NULL,
    cemail character varying(50) NOT NULL,
    cmobile character varying(11),
    eid integer NOT NULL,
    dataid integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (cid, cname, cage, cemail, cmobile, eid, dataid) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	08055185341	100	3
112	Arthur musa	50	a_musa@gmail.com	07055282813	107	10
113	philip akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	marylene mapa	33	m_mapa@gmail.com	08053333551	120	5
115	oghenero agor	50	o_agor@gmail.com	07055566774	117	11
116	adams bree	33	a_bree@gmail.com	08056765424	102	1
117	okafor mathias	45	o_mathias	08056763367	120	10
118	samson adeleke	65	s_adeleke@gmail.com	07056774423	117	11
119	lawal tamire	35	l_tamire@gmail.com	09052111101	107	5
120	james job	44	j_job@gmail.com	08059693919	100	8
121	matthew jakande	21	m_jakande@gmail.com	0705123144	120	2
122	jimila adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (cid);


--
-- PostgreSQL database dump complete
--

\unrestrict 2P4IIb7LBBWhnq8wTNrIarOe4ywCYoUNTvJdVwBZcgsXkNnVVC2NhbviZqyoLEd

