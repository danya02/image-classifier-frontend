import './App.scss';
import React from 'react';
import { Routes, Route, BrowserRouter } from 'react-router-dom';


import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonToolbar from 'react-bootstrap/ButtonToolbar';
import { LinkContainer } from 'react-router-bootstrap';

const Home = () => <><span>Home</span></>;

const About = () => <><span>About</span></>;

const Users = () => <><span>Users</span></>;


function App() {
  return (
    <BrowserRouter>
    <Container className="p-3">
      <Container className="p-5 mb-4 bg-light rounded-3">
        <h1 className="header">Welcome To React-Bootstrap</h1>
        <h2>
          Current Page is{' '}
          <Routes>
            <Route path="/about" element={<About />} />
            <Route path="/users" element={<Users />} />
            <Route path="/" element={<Home />}/>
          </Routes>
        </h2>
        <h2>
          Navigate to{' '}
          <ButtonToolbar className="custom-btn-toolbar">
            <LinkContainer to="/">
              <Button>Home</Button>
            </LinkContainer>
            <LinkContainer to="/about">
              <Button>About</Button>
            </LinkContainer>
            <LinkContainer to="/users">
              <Button>Users</Button>
            </LinkContainer>
          </ButtonToolbar>
        </h2>
      </Container>
    </Container>
  </BrowserRouter>
  );
}

export default App;
