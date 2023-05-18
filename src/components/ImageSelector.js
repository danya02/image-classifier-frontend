import { Card, FormControl } from "react-bootstrap";
import { useState } from "react";
import { InputGroup } from "react-bootstrap";
import Form from 'react-bootstrap/Form';
import React from "react";

export default function ({ onImgData, showPreview }) {
        const [picture, setPicture] = useState(null);
        const [imgData, setImgData] = useState(null);
        const onChangePicture = e => {
            if (e.target.files[0]) {
                console.log("picture: ", e.target.files);
                setPicture(e.target.files[0]);
                const reader = new FileReader();
                reader.addEventListener("load", () => {
                    setImgData(reader.result);
                    onImgData(reader.result);
                });
                reader.readAsDataURL(e.target.files[0]);
            }
        };
        if(showPreview){
            return (
                <Card>
                    <Form.Group controlId="formFile" className="mb-3" >
                        <Form.Label>Default file input example</Form.Label>
                        <Form.Control type="file" onChange={onChangePicture} />
                    </Form.Group>
                    <img className="playerProfilePic_home_tile" src={imgData} style={{width: "18rem"}} />
                </Card>
            );
        } else {
            return (
                <Card>
                    <Form.Group controlId="formFile" className="mb-3" >
                        <Form.Label>Default file input example</Form.Label>
                        <Form.Control type="file" onChange={onChangePicture} />
                    </Form.Group>
                </Card>
            );
        }
}
