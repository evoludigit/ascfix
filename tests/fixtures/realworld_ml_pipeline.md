# Real World: Machine Learning Pipeline
# Tests ML workflow diagram patterns

┌─────────────────┐
│   Data Sources  │
│                 │
│ - User Behavior │
│ - System Logs   │
│ - External APIs │
│ - Sensors       │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐    ┌─────────────────┐
│   Data Ingestion│    │   Data Cleaning │
│                 │    │                 │
│ - Streaming     │    │ - Missing Values│
│ - Batch         │    │ - Outliers      │
│ - Real-time     │    │ - Normalization │
└─────────┬───────┘    └─────────┬───────┘
          │                      │
          └─────────┬────────────┘
                    │
                    ▼
┌─────────────────┐    ┌─────────────────┐
│ Feature Engineering│ │   Model Training │
│                    │ │                  │
│ - Feature Selection│ │ - Algorithm      │
│ - Transformation   │ │   Selection     │
│ - Encoding         │ │ - Hyperparameter │
│                    │ │   Tuning        │
└─────────┬─────────┘ └─────────┬────────┘
          │                      │
          └─────────┬────────────┘
                    │
                    ▼
┌─────────────────┐    ┌─────────────────┐
│   Model Serving │    │ Model Monitoring│
│                 │ │                  │
│ - REST API      │ │ - Performance     │
│ - Batch Scoring │ │ - Drift Detection │
│ - Real-time     │ │ - Accuracy        │
└─────────────────┘    └─────────────────┘

# ML Pipeline Stages
1. **Data Collection**: Gather raw data from multiple sources
2. **Data Preparation**: Clean, transform, and feature engineer
3. **Model Development**: Train, validate, and tune models
4. **Model Deployment**: Serve predictions via APIs
5. **Model Monitoring**: Track performance and retrain as needed

# Common ML Algorithms
| Type | Algorithm | Use Case |
|------|-----------|----------|
| Classification | Random Forest | Fraud Detection |
| Regression | Linear Regression | Price Prediction |
| Clustering | K-Means | Customer Segmentation |
| Neural Networks | CNN | Image Recognition |
| Recommendation | Matrix Factorization | Product Recommendations |