// Code generated by ent, DO NOT EDIT.

package ent

import (
	"fmt"
	"strings"

	"entgo.io/ent"
	"entgo.io/ent/dialect/sql"
	"github.com/chroma-core/chroma/go/pkg/coordinator/ent/base"
	"github.com/google/uuid"
)

// Base is the model entity for the Base schema.
type Base struct {
	config `json:"-"`
	// ID of the ent.
	ID uuid.UUID `json:"id,omitempty"`
	// ParentID holds the value of the "parent_id" field.
	ParentID uuid.UUID `json:"parent_id,omitempty"`
	// Name holds the value of the "name" field.
	Name string `json:"name,omitempty"`
	// CreatedAt holds the value of the "created_at" field.
	CreatedAt uint `json:"created_at,omitempty"`
	// UpdatedAt holds the value of the "updated_at" field.
	UpdatedAt uint `json:"updated_at,omitempty"`
	// DeletedAt holds the value of the "deleted_at" field.
	DeletedAt uint `json:"deleted_at,omitempty"`
	// Version holds the value of the "version" field.
	Version      int `json:"version,omitempty"`
	selectValues sql.SelectValues
}

// scanValues returns the types for scanning values from sql.Rows.
func (*Base) scanValues(columns []string) ([]any, error) {
	values := make([]any, len(columns))
	for i := range columns {
		switch columns[i] {
		case base.FieldCreatedAt, base.FieldUpdatedAt, base.FieldDeletedAt, base.FieldVersion:
			values[i] = new(sql.NullInt64)
		case base.FieldName:
			values[i] = new(sql.NullString)
		case base.FieldID, base.FieldParentID:
			values[i] = new(uuid.UUID)
		default:
			values[i] = new(sql.UnknownType)
		}
	}
	return values, nil
}

// assignValues assigns the values that were returned from sql.Rows (after scanning)
// to the Base fields.
func (b *Base) assignValues(columns []string, values []any) error {
	if m, n := len(values), len(columns); m < n {
		return fmt.Errorf("mismatch number of scan values: %d != %d", m, n)
	}
	for i := range columns {
		switch columns[i] {
		case base.FieldID:
			if value, ok := values[i].(*uuid.UUID); !ok {
				return fmt.Errorf("unexpected type %T for field id", values[i])
			} else if value != nil {
				b.ID = *value
			}
		case base.FieldParentID:
			if value, ok := values[i].(*uuid.UUID); !ok {
				return fmt.Errorf("unexpected type %T for field parent_id", values[i])
			} else if value != nil {
				b.ParentID = *value
			}
		case base.FieldName:
			if value, ok := values[i].(*sql.NullString); !ok {
				return fmt.Errorf("unexpected type %T for field name", values[i])
			} else if value.Valid {
				b.Name = value.String
			}
		case base.FieldCreatedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field created_at", values[i])
			} else if value.Valid {
				b.CreatedAt = uint(value.Int64)
			}
		case base.FieldUpdatedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field updated_at", values[i])
			} else if value.Valid {
				b.UpdatedAt = uint(value.Int64)
			}
		case base.FieldDeletedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field deleted_at", values[i])
			} else if value.Valid {
				b.DeletedAt = uint(value.Int64)
			}
		case base.FieldVersion:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field version", values[i])
			} else if value.Valid {
				b.Version = int(value.Int64)
			}
		default:
			b.selectValues.Set(columns[i], values[i])
		}
	}
	return nil
}

// Value returns the ent.Value that was dynamically selected and assigned to the Base.
// This includes values selected through modifiers, order, etc.
func (b *Base) Value(name string) (ent.Value, error) {
	return b.selectValues.Get(name)
}

// Update returns a builder for updating this Base.
// Note that you need to call Base.Unwrap() before calling this method if this Base
// was returned from a transaction, and the transaction was committed or rolled back.
func (b *Base) Update() *BaseUpdateOne {
	return NewBaseClient(b.config).UpdateOne(b)
}

// Unwrap unwraps the Base entity that was returned from a transaction after it was closed,
// so that all future queries will be executed through the driver which created the transaction.
func (b *Base) Unwrap() *Base {
	_tx, ok := b.config.driver.(*txDriver)
	if !ok {
		panic("ent: Base is not a transactional entity")
	}
	b.config.driver = _tx.drv
	return b
}

// String implements the fmt.Stringer.
func (b *Base) String() string {
	var builder strings.Builder
	builder.WriteString("Base(")
	builder.WriteString(fmt.Sprintf("id=%v, ", b.ID))
	builder.WriteString("parent_id=")
	builder.WriteString(fmt.Sprintf("%v", b.ParentID))
	builder.WriteString(", ")
	builder.WriteString("name=")
	builder.WriteString(b.Name)
	builder.WriteString(", ")
	builder.WriteString("created_at=")
	builder.WriteString(fmt.Sprintf("%v", b.CreatedAt))
	builder.WriteString(", ")
	builder.WriteString("updated_at=")
	builder.WriteString(fmt.Sprintf("%v", b.UpdatedAt))
	builder.WriteString(", ")
	builder.WriteString("deleted_at=")
	builder.WriteString(fmt.Sprintf("%v", b.DeletedAt))
	builder.WriteString(", ")
	builder.WriteString("version=")
	builder.WriteString(fmt.Sprintf("%v", b.Version))
	builder.WriteByte(')')
	return builder.String()
}

// Bases is a parsable slice of Base.
type Bases []*Base