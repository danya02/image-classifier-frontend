import { Button, Card, FormControl } from "react-bootstrap";
import { useState } from "react";
import { InputGroup } from "react-bootstrap";
import Form from 'react-bootstrap/Form';
import React from "react";

export default function ({ src }) {
    return (
        <Card>
            <Card.Img variant="top" src={src} />
            <Card.Body>
                <Button variant="danger">Analyze</Button>
            </Card.Body>
        </Card>
    );
}
