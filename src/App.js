import './App.scss';
import React from 'react';
import { Routes, Route, BrowserRouter } from 'react-router-dom';


import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonToolbar from 'react-bootstrap/ButtonToolbar';
import { LinkContainer } from 'react-router-bootstrap';
import Home from './routes/Home.js';


function App() {
  return (
    <BrowserRouter>
      <Container className="p-3">
        <Container className="p-5 mb-4 bg-light rounded-3">
          <h1 className="header">Welcome To React-Bootstrap</h1>
          <h2>
            Navigate to{' '}
            <ButtonToolbar className="custom-btn-toolbar">
              <LinkContainer to="/">
                <Button>Home</Button>
              </LinkContainer>
            </ButtonToolbar>
          </h2>
        </Container>
        <Container>
          <Routes>
              <Route path="/" element={<Home />}/>
            </Routes>
        </Container>
      </Container>
    </BrowserRouter>
  );
}

export default App;
