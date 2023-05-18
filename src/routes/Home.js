import ImageSelector from '../components/ImageSelector';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import { useState } from "react";
import ImageViewer from '../components/ImageViewer';


export default function Home() {
    const [imgData, setImgData] = useState(null);

    return (
        <Container>
            <Row>
                <Col>
                    <ImageSelector onImgData={setImgData} showPreview={false} />
                </Col>

                <Col>
                    <ImageViewer src={imgData} />
                </Col>
            </Row>
        </Container>
        );
}