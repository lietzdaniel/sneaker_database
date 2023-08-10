// ShoeList.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './ShoeList.css';
import './ShoeOverlay.css';
//import ShoeOverlay from './ShoeOverlay';

function ShoeList() {
  const [shoes, setShoes] = useState([]);
  const [selectedShoe, setSelectedShoe] = useState(null);
  function handleShoeClick(shoe) {
    console.log('Clicked shoe:', shoe);
    setSelectedShoe(shoe);
  }
  useEffect(() => {
    axios.get('http://127.0.0.1:8000/shoes/api')
      .then(response => {
        setShoes(response.data);
      })
      .catch(error => {
        console.error('Error fetching data:', error);
      });
  }, []);
  const [imageLoaded, setImageLoaded] = React.useState(true);

  function ShoeOverlay({ shoe, onClose }) {
    const [prices, setPrices] = useState(null);
    const [lastSale, setLastSale] = useState(0);
    useEffect(() => {
  
      axios.get(`http://127.0.0.1:8000/shoes/api/json/${shoe.style_id}`)
        .then(response => {
          console.log('Fetched JSON data:', response.data);
          setPrices(response.data); // Set JSON data to the state
        })
        .catch(error => {
          console.error('Error fetching data:', error);
        });
    }, [shoe.style_id]);
   
    useEffect(() => {
      if (prices != null){
      if (prices.variants) {
        prices.variants.forEach(function (variant) {
          var sizeChart = variant.sizeChart;
          sizeChart.displayOptions.forEach(function (display) {
            if (display.type === "eu") {
             
              if (display.size === "EU " + shoe.size) {
                if (variant.market.salesInformation) {
                  setLastSale(variant.market.salesInformation.lastSale);
                }
              } 
            }
          });
        });
      }
    }}, [prices,shoe.size]);
  
    
    const gifImage = new Image();
    gifImage.src = 'http://127.0.0.1:8000/shoes/api/gif/' + shoe.style_id;
    gifImage.onload = () => {

      setImageLoaded(true);
    };
    gifImage.onerror = () => {

      setImageLoaded(false);
    };
    
    return (
      <div className="overlay">
        <div className="overlay-content">
          <button className="close-button" onClick={onClose}>
            Close
          </button>
          
          {imageLoaded ? (
            <img src={'http://127.0.0.1:8000/shoes/api/gif/' + shoe.style_id} alt={shoe.name} />
          ) : (
            <img src={shoe.image} alt={shoe.name} />
          )}
          { <div style={{textAlign: 'left'}} className="shoe-information">
              <a href={"https://stockx.com" + shoe.link}>
              <h1 style={{ fontSize: 50 ,fontWeight: 'bold'} }>{shoe.shoe_type}</h1>
              <p style={{ fontSize: 50,fontWeight: 'bold' } }>{shoe.model}</p>
              </a>
              <p style={{ fontSize: 50 }}>Style: {shoe.style_id}</p>
              <p style={{ fontSize: 50 }}>Retail Price: {shoe.retail_price}</p>
              <p style={{ fontSize: 50 }}>Your Size: {shoe.size}</p>
              <p style={{ fontSize: 50 }}>Last Sale: {lastSale}€</p>
            </div>
            }
        </div>
      </div>
    );
  }

  return (
    <div className="shoe-list-container">
      <div className="shoe-list">
        {shoes.map(shoe => (
          <div className="shoe-container" key={shoe.id}>
            <img
              src={shoe.image}
              alt={shoe.name}
              onClick={() => handleShoeClick(shoe)} // Add this line
            />
          </div>
        ))}
      </div>
      {selectedShoe && (
        <ShoeOverlay shoe={selectedShoe} onClose={() => setSelectedShoe(null)} />
      )}
    </div>
  );
}

export default ShoeList;