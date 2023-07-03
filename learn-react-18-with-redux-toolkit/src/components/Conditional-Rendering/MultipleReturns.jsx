import React, { useState, useEffect } from "react";

const MultipleReturns = () => {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setTimeout(() => {
      setLoading(false);
    }, 2000);
  }, []);

  if (loading) {
    return <h2>Loading...</h2>;
  }

  return <h2>Loaded</h2>;
};

export default MultipleReturns;
